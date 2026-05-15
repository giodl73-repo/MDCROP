use rctx_core::RCTX_VERSION;
use rgraph_core::{
    shortest_connector_path, undirected_boundary_metrics, undirected_cluster_summaries,
};
use ropt_core::{exact_budgeted_selection, BudgetItem, RoptError, SeedPart};
use rstat_core::scoring::{weighted_component_score, ScoringError};
use rstat_core::summary::{summary_stats, SummaryError};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use thiserror::Error;

pub const COMPETITOR_RESULTS_SCHEMA: &str = "crop.competitor-results.v1";
pub const BENCH_MANIFEST_SCHEMA: &str = "crop.bench-manifest.v1";
pub const CROP_VIEW_SCHEMA: &str = "crop.view.v1";

#[derive(Debug, Error)]
pub enum CropError {
    #[error("[INPUT] evidence graph must contain at least one unit")]
    EmptyGraph,
    #[error("[INPUT] seed unit {seed} is out of bounds for graph with {node_count} units")]
    SeedOutOfBounds { seed: usize, node_count: usize },
    #[error("[INPUT] adjacency length {adjacency_len} does not match unit count {unit_count}")]
    AdjacencyLengthMismatch {
        adjacency_len: usize,
        unit_count: usize,
    },
    #[error(
        "[INPUT] unit {unit} references neighbor {neighbor}, but graph has {node_count} units"
    )]
    NeighborOutOfBounds {
        unit: usize,
        neighbor: usize,
        node_count: usize,
    },
    #[error("[INPUT] relevance score for unit {unit} is non-finite: {score}")]
    NonFiniteRelevance { unit: usize, score: f64 },
    #[error("[INPUT] budget must be positive")]
    NonPositiveBudget,
    #[error("[INPUT] root path does not exist: {path}")]
    RootDoesNotExist { path: String },
    #[error("[INPUT] root path has no supported evidence files: {path}")]
    NoSupportedFiles { path: String },
    #[error("[INPUT] failed to read evidence file {path}: {source}")]
    ReadFile {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("[INPUT] evidence file is not valid UTF-8: {path}")]
    NonUtf8File { path: String },
    #[error("[INPUT] failed to inspect path {path}: {source}")]
    InspectPath {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("[KERNEL] edge-cut failed: {0}")]
    EdgeCut(String),
    #[error("[KERNEL] summary failed: {0}")]
    Summary(#[from] SummaryError),
    #[error("[KERNEL] scoring failed: {0}")]
    Scoring(#[from] ScoringError),
    #[error("[KERNEL] budget packing failed: {0}")]
    Budget(#[from] RoptError),
    #[error("[KERNEL] cluster summary failed: {0}")]
    Cluster(String),
    #[error("[CACHE] failed to serialize prefix block: {0}")]
    PrefixSerialize(#[from] serde_json::Error),
    #[error("[EVAL] failed to parse competitor results: {0}")]
    CompetitorImport(serde_json::Error),
    #[error("[EVAL] unsupported competitor result schema: {schema_version}")]
    UnsupportedCompetitorSchema { schema_version: String },
    #[error("[EVAL] competitor result references unknown unit ID: {unit_id}")]
    UnknownCompetitorUnit { unit_id: String },
    #[error("[EVAL] competitor result repeats unit ID: {unit_id}")]
    DuplicateCompetitorUnit { unit_id: String },
    #[error("[BENCH] failed to parse benchmark manifest: {0}")]
    BenchManifestImport(serde_json::Error),
    #[error("[BENCH] unsupported benchmark manifest schema: {schema_version}")]
    UnsupportedBenchManifestSchema { schema_version: String },
    #[error("[BENCH] benchmark manifest must contain at least one case")]
    EmptyBenchManifest,
    #[error("[VIEW] failed to parse crop view: {0}")]
    ViewImport(serde_json::Error),
    #[error("[VIEW] unsupported crop view schema: {schema_version}")]
    UnsupportedViewSchema { schema_version: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceUnit {
    pub id: String,
    pub source_id: String,
    pub content_hash: String,
    pub text: String,
    pub relevance: f64,
    pub token_estimate: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceGraph {
    pub units: Vec<EvidenceUnit>,
    pub adjacency: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CropOptions {
    pub seed: usize,
    pub token_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CropUnit {
    pub id: String,
    pub source_id: String,
    pub content_hash: String,
    pub text: String,
    pub relevance: f64,
    pub token_estimate: usize,
    pub reason: CropReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngestReport {
    pub root: String,
    pub source_count: usize,
    pub unit_count: usize,
    pub edge_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IngestedEvidence {
    pub graph: EvidenceGraph,
    pub report: IngestReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CropReason {
    Seed,
    BridgeConnector,
    ConnectorTarget,
    RelevantNeighbor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CropPack {
    pub units: Vec<CropUnit>,
    pub representative_unit_id: String,
    pub token_estimate: usize,
    pub boundary_cut_edges: usize,
    pub boundary_conductance: f64,
    pub relevance_mean: f64,
    pub quality_score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrefixCacheProfile {
    Generic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixSourceEntry {
    pub source_id: String,
    pub content_hash: String,
    pub unit_count: usize,
    pub token_estimate: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrefixBlock {
    pub schema_version: String,
    pub profile: PrefixCacheProfile,
    pub engine: String,
    pub root: String,
    pub source_count: usize,
    pub unit_count: usize,
    pub edge_count: usize,
    pub sources: Vec<PrefixSourceEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskTail {
    pub task: String,
    pub seed: usize,
    pub token_budget: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrefixCachedPack {
    pub cache_key: String,
    pub prefix_hash: String,
    pub prefix: PrefixBlock,
    pub body: CropPack,
    pub task_tail: TaskTail,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchTimings {
    pub ingest_ms: u64,
    pub score_ms: u64,
    pub crop_ms: u64,
    pub total_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchReport {
    pub schema_version: String,
    pub root: String,
    pub task: String,
    pub token_budget: usize,
    pub seed: usize,
    pub source_count: usize,
    pub unit_count: usize,
    pub edge_count: usize,
    pub selected_unit_count: usize,
    pub selected_token_estimate: usize,
    pub boundary_cut_edges: usize,
    pub boundary_conductance: f64,
    pub relevance_mean: f64,
    pub quality_score: f64,
    pub timings: BenchTimings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchManifestCase {
    pub id: String,
    pub root: String,
    pub task: String,
    pub token_budget: usize,
    pub seed: usize,
    pub view: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BenchManifest {
    pub schema_version: String,
    pub cases: Vec<BenchManifestCase>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchManifestCaseReport {
    pub id: String,
    pub view: Option<String>,
    pub report: BenchReport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchAggregate {
    pub method: String,
    pub case_count: usize,
    pub total_source_count: usize,
    pub total_unit_count: usize,
    pub total_edge_count: usize,
    pub total_selected_token_estimate: usize,
    pub mean_selected_unit_count: f64,
    pub mean_selected_token_estimate: f64,
    pub mean_quality_score: f64,
    pub mean_ingest_ms: f64,
    pub mean_score_ms: f64,
    pub mean_crop_ms: f64,
    pub mean_total_ms: f64,
    pub max_total_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BenchManifestReport {
    pub schema_version: String,
    pub case_count: usize,
    pub aggregate: BenchAggregate,
    pub cases: Vec<BenchManifestCaseReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CropView {
    pub schema_version: String,
    pub name: String,
    pub root: String,
    pub task: String,
    pub token_budget: usize,
    pub seed: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CropViewRun {
    pub schema_version: String,
    pub name: String,
    pub root: String,
    pub task: String,
    pub token_budget: usize,
    pub seed: usize,
    pub pack: CropPack,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CropViewSummary {
    pub file: String,
    pub name: String,
    pub root: String,
    pub task: String,
    pub token_budget: usize,
    pub seed: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CropViewList {
    pub schema_version: String,
    pub store: String,
    pub view_count: usize,
    pub views: Vec<CropViewSummary>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvalCase {
    MissingBridge,
    NoisyPeriphery,
    StaleDuplicate,
    BudgetPacking,
    CrossLinkChain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvalMethod {
    Crop,
    ScoreTopK,
    LexicalTopK,
    HybridTopK,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalReport {
    pub case: EvalCase,
    pub method: EvalMethod,
    pub token_budget: usize,
    pub selected_unit_ids: Vec<String>,
    pub token_estimate: usize,
    pub gold_recall: f64,
    pub bridge_recall: f64,
    pub noise_rate: f64,
    pub source_coverage: f64,
    pub budget_efficiency: f64,
    pub noise_token_share: f64,
    pub quality_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalAggregate {
    pub method: EvalMethod,
    pub case_count: usize,
    pub mean_gold_recall: f64,
    pub mean_bridge_recall: f64,
    pub mean_noise_rate: f64,
    pub mean_source_coverage: f64,
    pub mean_budget_efficiency: f64,
    pub mean_noise_token_share: f64,
    pub mean_quality_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvalSuiteReport {
    pub token_budget: usize,
    pub cases: Vec<EvalCase>,
    pub methods: Vec<EvalMethod>,
    pub rows: Vec<EvalReport>,
    pub aggregates: Vec<EvalAggregate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompetitorClass {
    Vector,
    Lexical,
    Hybrid,
    RagOrchestration,
    Graph,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompetitorMethodMetadata {
    pub method_id: String,
    pub competitor_class: CompetitorClass,
    pub implementation: String,
    pub version: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitorSelectedUnit {
    pub unit_id: String,
    pub score: Option<f64>,
    pub token_estimate: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitorEvalInput {
    pub schema_version: String,
    pub case: EvalCase,
    pub token_budget: usize,
    pub method: CompetitorMethodMetadata,
    pub selected_units: Vec<CompetitorSelectedUnit>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitorEvalReport {
    pub schema_version: String,
    pub case: EvalCase,
    pub method: CompetitorMethodMetadata,
    pub token_budget: usize,
    pub selected_unit_ids: Vec<String>,
    pub token_estimate: usize,
    pub reported_token_estimate: Option<usize>,
    pub gold_recall: f64,
    pub bridge_recall: f64,
    pub noise_rate: f64,
    pub source_coverage: f64,
    pub budget_efficiency: f64,
    pub noise_token_share: f64,
    pub quality_score: f64,
}

pub trait CompetitorAdapter {
    fn export_competitor_results(&self) -> CompetitorEvalInput;
}

pub fn shared_kernel_probe(seed: usize) -> (&'static str, SeedPart) {
    (RCTX_VERSION, SeedPart::Usize(seed))
}

pub fn fixture_graph() -> EvidenceGraph {
    EvidenceGraph {
        units: vec![
            unit_with_source(
                "task-seed",
                "fixture:task",
                "Explain the target note.",
                1.0,
                80,
            ),
            unit_with_source(
                "bridge-note",
                "fixture:note",
                "Bridge note connecting the task to the target evidence.",
                0.2,
                40,
            ),
            unit_with_source(
                "target-note",
                "fixture:target",
                "Target note with the answer evidence.",
                0.95,
                40,
            ),
            unit_with_source(
                "stale-noise",
                "fixture:old",
                "Old noisy note that should stay outside the crop.",
                0.9,
                40,
            ),
        ],
        adjacency: vec![vec![1, 3], vec![0, 2], vec![1], vec![0]],
    }
}

pub fn eval_fixture(
    case: EvalCase,
    method: EvalMethod,
    token_budget: usize,
) -> Result<EvalReport, CropError> {
    if token_budget == 0 {
        return Err(CropError::NonPositiveBudget);
    }
    eval_case(fixture_for_case(case), method, token_budget)
}

pub fn eval_suite(token_budget: usize) -> Result<EvalSuiteReport, CropError> {
    if token_budget == 0 {
        return Err(CropError::NonPositiveBudget);
    }
    let cases = all_eval_cases();
    let methods = all_eval_methods();
    let mut rows = Vec::new();
    for case in &cases {
        for method in &methods {
            rows.push(eval_fixture(*case, *method, token_budget)?);
        }
    }
    let aggregates = methods
        .iter()
        .map(|&method| aggregate_method(method, &rows))
        .collect::<Vec<_>>();
    Ok(EvalSuiteReport {
        token_budget,
        cases,
        methods,
        rows,
        aggregates,
    })
}

pub fn eval_competitor_json(json: &str) -> Result<CompetitorEvalReport, CropError> {
    let input = serde_json::from_str(json).map_err(|source| CropError::CompetitorImport(source))?;
    eval_competitor_results(input)
}

pub fn eval_competitor_results(
    input: CompetitorEvalInput,
) -> Result<CompetitorEvalReport, CropError> {
    if input.token_budget == 0 {
        return Err(CropError::NonPositiveBudget);
    }
    if input.schema_version != COMPETITOR_RESULTS_SCHEMA {
        return Err(CropError::UnsupportedCompetitorSchema {
            schema_version: input.schema_version,
        });
    }
    let fixture = fixture_for_case(input.case);
    let selected = validate_competitor_selection(&fixture.graph, &input.selected_units)?;
    let report = eval_selected_units(&fixture, &selected, input.token_budget, None)?;
    let reported_token_estimate = input
        .selected_units
        .iter()
        .map(|unit| unit.token_estimate)
        .try_fold(0usize, |sum, token_estimate| {
            token_estimate.map(|token_estimate| sum + token_estimate)
        });

    Ok(CompetitorEvalReport {
        schema_version: input.schema_version,
        case: input.case,
        method: input.method,
        token_budget: input.token_budget,
        selected_unit_ids: report.selected_unit_ids,
        token_estimate: report.token_estimate,
        reported_token_estimate,
        gold_recall: report.gold_recall,
        bridge_recall: report.bridge_recall,
        noise_rate: report.noise_rate,
        source_coverage: report.source_coverage,
        budget_efficiency: report.budget_efficiency,
        noise_token_share: report.noise_token_share,
        quality_score: report.quality_score,
    })
}

pub fn all_eval_cases() -> Vec<EvalCase> {
    vec![
        EvalCase::MissingBridge,
        EvalCase::NoisyPeriphery,
        EvalCase::StaleDuplicate,
        EvalCase::BudgetPacking,
        EvalCase::CrossLinkChain,
    ]
}

pub fn all_eval_methods() -> Vec<EvalMethod> {
    vec![
        EvalMethod::Crop,
        EvalMethod::ScoreTopK,
        EvalMethod::LexicalTopK,
        EvalMethod::HybridTopK,
    ]
}

pub fn prefix_cached_pack(
    ingested: &IngestedEvidence,
    body: CropPack,
    task: impl Into<String>,
    options: CropOptions,
    profile: PrefixCacheProfile,
) -> Result<PrefixCachedPack, CropError> {
    let prefix = prefix_block(ingested, profile);
    let prefix_bytes = serde_json::to_vec(&prefix)?;
    let prefix_hash = sha256_hex(&prefix_bytes);
    let cache_key = format!(
        "crop-prefix:{}:{}",
        prefix_profile_name(profile),
        prefix_hash.trim_start_matches("sha256:")
    );
    Ok(PrefixCachedPack {
        cache_key,
        prefix_hash,
        prefix,
        body,
        task_tail: TaskTail {
            task: task.into(),
            seed: options.seed,
            token_budget: options.token_budget,
        },
    })
}

pub fn bench_pack(
    root: impl AsRef<Path>,
    task: &str,
    options: CropOptions,
) -> Result<BenchReport, CropError> {
    if options.token_budget == 0 {
        return Err(CropError::NonPositiveBudget);
    }

    let total_start = Instant::now();

    let ingest_start = Instant::now();
    let mut ingested = ingest_path(root)?;
    let ingest_ms = elapsed_ms(ingest_start);

    let score_start = Instant::now();
    score_graph_for_task(&mut ingested.graph, task);
    let score_ms = elapsed_ms(score_start);

    let crop_start = Instant::now();
    let pack = crop_context(&ingested.graph, options.clone())?;
    let crop_ms = elapsed_ms(crop_start);
    let total_ms = elapsed_ms(total_start);

    Ok(BenchReport {
        schema_version: "crop.bench.v1".to_string(),
        root: ingested.report.root,
        task: task.to_string(),
        token_budget: options.token_budget,
        seed: options.seed,
        source_count: ingested.report.source_count,
        unit_count: ingested.report.unit_count,
        edge_count: ingested.report.edge_count,
        selected_unit_count: pack.units.len(),
        selected_token_estimate: pack.token_estimate,
        boundary_cut_edges: pack.boundary_cut_edges,
        boundary_conductance: pack.boundary_conductance,
        relevance_mean: pack.relevance_mean,
        quality_score: pack.quality_score,
        timings: BenchTimings {
            ingest_ms,
            score_ms,
            crop_ms,
            total_ms,
        },
    })
}

pub fn bench_manifest_json(
    json: &str,
    base_dir: impl AsRef<Path>,
) -> Result<BenchManifestReport, CropError> {
    let manifest =
        serde_json::from_str(json).map_err(|source| CropError::BenchManifestImport(source))?;
    bench_manifest(manifest, base_dir)
}

pub fn bench_manifest(
    manifest: BenchManifest,
    base_dir: impl AsRef<Path>,
) -> Result<BenchManifestReport, CropError> {
    if manifest.schema_version != BENCH_MANIFEST_SCHEMA {
        return Err(CropError::UnsupportedBenchManifestSchema {
            schema_version: manifest.schema_version,
        });
    }
    if manifest.cases.is_empty() {
        return Err(CropError::EmptyBenchManifest);
    }

    let base_dir = base_dir.as_ref();
    let mut cases = Vec::new();
    for case in manifest.cases {
        let root = manifest_root(base_dir, &case.root);
        let report = bench_pack(
            root,
            &case.task,
            CropOptions {
                seed: case.seed,
                token_budget: case.token_budget,
            },
        )?;
        cases.push(BenchManifestCaseReport {
            id: case.id,
            view: case.view,
            report,
        });
    }

    Ok(BenchManifestReport {
        schema_version: "crop.bench-report.v1".to_string(),
        case_count: cases.len(),
        aggregate: aggregate_bench_cases(&cases),
        cases,
    })
}

pub fn run_view_json(json: &str, base_dir: impl AsRef<Path>) -> Result<CropViewRun, CropError> {
    let view = serde_json::from_str(json).map_err(|source| CropError::ViewImport(source))?;
    run_view(view, base_dir)
}

pub fn run_view(view: CropView, base_dir: impl AsRef<Path>) -> Result<CropViewRun, CropError> {
    if view.schema_version != CROP_VIEW_SCHEMA {
        return Err(CropError::UnsupportedViewSchema {
            schema_version: view.schema_version,
        });
    }
    if view.token_budget == 0 {
        return Err(CropError::NonPositiveBudget);
    }

    let root = manifest_root(base_dir.as_ref(), &view.root);
    let mut ingested = ingest_path(root)?;
    score_graph_for_task(&mut ingested.graph, &view.task);
    let pack = crop_context(
        &ingested.graph,
        CropOptions {
            seed: view.seed,
            token_budget: view.token_budget,
        },
    )?;

    Ok(CropViewRun {
        schema_version: "crop.view-run.v1".to_string(),
        name: view.name,
        root: view.root,
        task: view.task,
        token_budget: view.token_budget,
        seed: view.seed,
        pack,
    })
}

pub fn list_views(store: impl AsRef<Path>) -> Result<CropViewList, CropError> {
    let store = store.as_ref();
    if !store.exists() {
        return Err(CropError::RootDoesNotExist {
            path: store.display().to_string(),
        });
    }

    let mut entries = fs::read_dir(store)
        .map_err(|source| CropError::InspectPath {
            path: store.display().to_string(),
            source,
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| CropError::InspectPath {
            path: store.display().to_string(),
            source,
        })?;
    entries.sort_by_key(|entry| entry.path());

    let mut views = Vec::new();
    for entry in entries {
        let path = entry.path();
        if !matches!(path.extension().and_then(|ext| ext.to_str()), Some("json")) {
            continue;
        }
        let json = fs::read_to_string(&path).map_err(|source| CropError::ReadFile {
            path: path.display().to_string(),
            source,
        })?;
        let view: CropView =
            serde_json::from_str(&json).map_err(|source| CropError::ViewImport(source))?;
        if view.schema_version != CROP_VIEW_SCHEMA {
            return Err(CropError::UnsupportedViewSchema {
                schema_version: view.schema_version,
            });
        }
        views.push(CropViewSummary {
            file: path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| path.display().to_string()),
            name: view.name,
            root: view.root,
            task: view.task,
            token_budget: view.token_budget,
            seed: view.seed,
        });
    }

    Ok(CropViewList {
        schema_version: "crop.view-list.v1".to_string(),
        store: store.display().to_string(),
        view_count: views.len(),
        views,
    })
}

pub fn ingest_path(root: impl AsRef<Path>) -> Result<IngestedEvidence, CropError> {
    let root = root.as_ref();
    if !root.exists() {
        return Err(CropError::RootDoesNotExist {
            path: root.display().to_string(),
        });
    }

    let files = supported_files(root)?;
    if files.is_empty() {
        return Err(CropError::NoSupportedFiles {
            path: root.display().to_string(),
        });
    }

    let mut units = Vec::new();
    let mut source_first_unit = BTreeMap::new();
    let mut source_unit_indices = BTreeMap::<String, Vec<usize>>::new();
    let mut source_links = BTreeMap::<String, Vec<String>>::new();

    for file in &files {
        let bytes = fs::read(file).map_err(|source| CropError::ReadFile {
            path: file.display().to_string(),
            source,
        })?;
        let text = String::from_utf8(bytes).map_err(|_| CropError::NonUtf8File {
            path: file.display().to_string(),
        })?;
        let source_id = source_id(root, file);
        let source_hash = sha256_hex(text.as_bytes());
        let chunks = text_chunks(&text);
        source_links.insert(source_id.clone(), markdown_links(root, file, &text));

        for (chunk_index, chunk) in chunks.into_iter().enumerate() {
            let unit_index = units.len();
            source_first_unit
                .entry(source_id.clone())
                .or_insert(unit_index);
            source_unit_indices
                .entry(source_id.clone())
                .or_default()
                .push(unit_index);
            units.push(EvidenceUnit {
                id: format!("{source_id}#{chunk_index:03}"),
                source_id: source_id.clone(),
                content_hash: source_hash.clone(),
                text: chunk.to_string(),
                relevance: 0.0,
                token_estimate: estimate_tokens(chunk),
            });
        }
    }

    let mut edge_sets = vec![BTreeSet::new(); units.len()];
    for indices in source_unit_indices.values() {
        for pair in indices.windows(2) {
            add_undirected_edge(&mut edge_sets, pair[0], pair[1]);
        }
    }
    for (source_id, links) in source_links {
        let Some(from_indices) = source_unit_indices.get(&source_id) else {
            continue;
        };
        for target_source in links {
            if let Some(&target_index) = source_first_unit.get(&target_source) {
                for &from_index in from_indices {
                    add_undirected_edge(&mut edge_sets, from_index, target_index);
                }
            }
        }
    }

    let adjacency = edge_sets
        .into_iter()
        .map(|neighbors| neighbors.into_iter().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let edge_count = adjacency.iter().map(Vec::len).sum::<usize>() / 2;
    let source_count = source_unit_indices.len();
    let unit_count = units.len();

    Ok(IngestedEvidence {
        graph: EvidenceGraph { units, adjacency },
        report: IngestReport {
            root: root.display().to_string(),
            source_count,
            unit_count,
            edge_count,
        },
    })
}

pub fn score_graph_for_task(graph: &mut EvidenceGraph, task: &str) {
    let terms = task_terms(task);
    for unit in &mut graph.units {
        unit.relevance = relevance_score(unit, &terms);
    }
}

pub fn crop_context(graph: &EvidenceGraph, options: CropOptions) -> Result<CropPack, CropError> {
    validate_graph(graph)?;
    if options.token_budget == 0 {
        return Err(CropError::NonPositiveBudget);
    }
    if options.seed >= graph.units.len() {
        return Err(CropError::SeedOutOfBounds {
            seed: options.seed,
            node_count: graph.units.len(),
        });
    }

    let mut selected = vec![false; graph.units.len()];
    selected[options.seed] = true;
    let mut token_estimate = graph.units[options.seed].token_estimate;
    let mut reasons = vec![None; graph.units.len()];
    reasons[options.seed] = Some(CropReason::Seed);

    if let Some(target) = highest_relevance_target(graph, options.seed) {
        if let Some(path) = shortest_connector_path(&graph.adjacency, &[options.seed], &[target])
            .map_err(|err| CropError::EdgeCut(err.to_string()))?
        {
            let path_token_estimate = path
                .nodes
                .iter()
                .copied()
                .filter(|&node| node != options.seed)
                .map(|node| graph.units[node].token_estimate)
                .sum::<usize>();
            if token_estimate + path_token_estimate <= options.token_budget {
                for &node in &path.bridge_nodes {
                    selected[node] = true;
                    reasons[node] = Some(CropReason::BridgeConnector);
                }
                if path.target != options.seed {
                    selected[path.target] = true;
                    reasons[path.target] = Some(CropReason::ConnectorTarget);
                }
                token_estimate += path_token_estimate;
            }
        }
    }

    let remaining_budget = options.token_budget.saturating_sub(token_estimate);
    let mut neighbor_candidates = graph.adjacency[options.seed]
        .iter()
        .copied()
        .filter(|&neighbor| !selected[neighbor])
        .collect::<Vec<_>>();
    neighbor_candidates.sort_unstable();
    neighbor_candidates.dedup();
    let budget_items = neighbor_candidates
        .iter()
        .map(|&neighbor| BudgetItem {
            score: graph.units[neighbor].relevance,
            cost: graph.units[neighbor].token_estimate,
        })
        .collect::<Vec<_>>();
    if remaining_budget > 0
        && budget_items
            .iter()
            .any(|item| item.cost <= remaining_budget)
    {
        let packed = exact_budgeted_selection(&budget_items, remaining_budget)?;
        for packed_index in packed.selected_indices {
            let neighbor = neighbor_candidates[packed_index];
            selected[neighbor] = true;
            reasons[neighbor] = Some(CropReason::RelevantNeighbor);
            token_estimate += graph.units[neighbor].token_estimate;
        }
    }

    let boundary_metrics = undirected_boundary_metrics(&graph.adjacency, &selected)
        .map_err(|err| CropError::EdgeCut(err.to_string()))?;

    let mut units = Vec::new();
    let mut relevance = Vec::new();
    for (index, unit) in graph.units.iter().enumerate() {
        if selected[index] {
            relevance.push(unit.relevance);
            units.push(CropUnit {
                id: unit.id.clone(),
                source_id: unit.source_id.clone(),
                content_hash: unit.content_hash.clone(),
                text: unit.text.clone(),
                relevance: unit.relevance,
                token_estimate: unit.token_estimate,
                reason: reasons[index].expect("selected crop unit has reason"),
            });
        }
    }

    let selected_indices = selected
        .iter()
        .enumerate()
        .filter_map(|(index, is_selected)| is_selected.then_some(index))
        .collect::<Vec<_>>();
    let cluster_summary = undirected_cluster_summaries(&graph.adjacency, &[selected_indices])
        .map_err(|err| CropError::Cluster(err.to_string()))?
        .into_iter()
        .next()
        .expect("selected crop always includes seed");
    let representative_unit_id = graph.units[cluster_summary.representative_node].id.clone();

    let relevance_mean = summary_stats(&relevance)?.mean;
    let quality_score = weighted_component_score(
        &[relevance_mean, 1.0 - boundary_metrics.conductance],
        &[0.8, 0.2],
    )?;
    Ok(CropPack {
        units,
        representative_unit_id,
        token_estimate,
        boundary_cut_edges: boundary_metrics.boundary_edges,
        boundary_conductance: boundary_metrics.conductance,
        relevance_mean,
        quality_score,
    })
}

fn prefix_block(ingested: &IngestedEvidence, profile: PrefixCacheProfile) -> PrefixBlock {
    let mut sources = BTreeMap::<String, PrefixSourceEntry>::new();
    for unit in &ingested.graph.units {
        let entry = sources
            .entry(unit.source_id.clone())
            .or_insert_with(|| PrefixSourceEntry {
                source_id: unit.source_id.clone(),
                content_hash: unit.content_hash.clone(),
                unit_count: 0,
                token_estimate: 0,
            });
        entry.unit_count += 1;
        entry.token_estimate += unit.token_estimate;
    }

    PrefixBlock {
        schema_version: "crop.prefix.v1".to_string(),
        profile,
        engine: format!("crop-core/{}", env!("CARGO_PKG_VERSION")),
        root: ingested.report.root.clone(),
        source_count: ingested.report.source_count,
        unit_count: ingested.report.unit_count,
        edge_count: ingested.report.edge_count,
        sources: sources.into_values().collect(),
    }
}

fn prefix_profile_name(profile: PrefixCacheProfile) -> &'static str {
    match profile {
        PrefixCacheProfile::Generic => "generic",
    }
}

fn aggregate_method(method: EvalMethod, rows: &[EvalReport]) -> EvalAggregate {
    let matching = rows
        .iter()
        .filter(|row| row.method == method)
        .collect::<Vec<_>>();
    let case_count = matching.len();
    EvalAggregate {
        method,
        case_count,
        mean_gold_recall: mean_by(&matching, |row| row.gold_recall),
        mean_bridge_recall: mean_by(&matching, |row| row.bridge_recall),
        mean_noise_rate: mean_by(&matching, |row| row.noise_rate),
        mean_source_coverage: mean_by(&matching, |row| row.source_coverage),
        mean_budget_efficiency: mean_by(&matching, |row| row.budget_efficiency),
        mean_noise_token_share: mean_by(&matching, |row| row.noise_token_share),
        mean_quality_score: mean_by(&matching, |row| row.quality_score),
    }
}

fn aggregate_bench_cases(cases: &[BenchManifestCaseReport]) -> BenchAggregate {
    let case_count = cases.len();
    BenchAggregate {
        method: "crop".to_string(),
        case_count,
        total_source_count: cases
            .iter()
            .map(|case| case.report.source_count)
            .sum::<usize>(),
        total_unit_count: cases
            .iter()
            .map(|case| case.report.unit_count)
            .sum::<usize>(),
        total_edge_count: cases
            .iter()
            .map(|case| case.report.edge_count)
            .sum::<usize>(),
        total_selected_token_estimate: cases
            .iter()
            .map(|case| case.report.selected_token_estimate)
            .sum::<usize>(),
        mean_selected_unit_count: mean_bench_by(cases, |case| {
            case.report.selected_unit_count as f64
        }),
        mean_selected_token_estimate: mean_bench_by(cases, |case| {
            case.report.selected_token_estimate as f64
        }),
        mean_quality_score: mean_bench_by(cases, |case| case.report.quality_score),
        mean_ingest_ms: mean_bench_by(cases, |case| case.report.timings.ingest_ms as f64),
        mean_score_ms: mean_bench_by(cases, |case| case.report.timings.score_ms as f64),
        mean_crop_ms: mean_bench_by(cases, |case| case.report.timings.crop_ms as f64),
        mean_total_ms: mean_bench_by(cases, |case| case.report.timings.total_ms as f64),
        max_total_ms: cases
            .iter()
            .map(|case| case.report.timings.total_ms)
            .max()
            .unwrap_or(0),
    }
}

fn mean_by<F>(rows: &[&EvalReport], value: F) -> f64
where
    F: Fn(&EvalReport) -> f64,
{
    if rows.is_empty() {
        return 0.0;
    }
    rows.iter().map(|row| value(row)).sum::<f64>() / rows.len() as f64
}

fn mean_bench_by<F>(cases: &[BenchManifestCaseReport], value: F) -> f64
where
    F: Fn(&BenchManifestCaseReport) -> f64,
{
    if cases.is_empty() {
        return 0.0;
    }
    cases.iter().map(value).sum::<f64>() / cases.len() as f64
}

struct EvalFixture {
    case: EvalCase,
    task: &'static str,
    graph: EvidenceGraph,
    gold: Vec<&'static str>,
    bridge: Vec<&'static str>,
    noise: Vec<&'static str>,
}

fn fixture_for_case(case: EvalCase) -> EvalFixture {
    match case {
        EvalCase::MissingBridge => EvalFixture {
            case,
            task: "target evidence",
            graph: fixture_graph(),
            gold: vec!["task-seed", "bridge-note", "target-note"],
            bridge: vec!["bridge-note"],
            noise: vec!["stale-noise"],
        },
        EvalCase::NoisyPeriphery => EvalFixture {
            case,
            task: "target evidence",
            graph: EvidenceGraph {
                units: vec![
                    unit_with_source("seed", "fixture:task", "Task seed.", 1.0, 80),
                    unit_with_source("bridge", "fixture:bridge", "Relevant bridge.", 0.25, 40),
                    unit_with_source("target", "fixture:target", "Target evidence.", 0.95, 40),
                    unit_with_source(
                        "periphery-noise",
                        "fixture:noise",
                        "Nearby noisy aside.",
                        0.9,
                        40,
                    ),
                ],
                adjacency: vec![vec![1, 3], vec![0, 2], vec![1], vec![0]],
            },
            gold: vec!["seed", "bridge", "target"],
            bridge: vec!["bridge"],
            noise: vec!["periphery-noise"],
        },
        EvalCase::StaleDuplicate => EvalFixture {
            case,
            task: "current canonical task seed",
            graph: EvidenceGraph {
                units: vec![
                    unit_with_source("seed", "fixture:task", "Task seed.", 1.0, 60),
                    unit_with_source(
                        "current-canonical",
                        "fixture:current",
                        "Current canonical evidence.",
                        0.85,
                        50,
                    ),
                    unit_with_source(
                        "stale-duplicate",
                        "fixture:stale",
                        "Stale duplicate with misleading score.",
                        0.99,
                        50,
                    ),
                ],
                adjacency: vec![vec![1], vec![0], vec![]],
            },
            gold: vec!["seed", "current-canonical"],
            bridge: vec![],
            noise: vec!["stale-duplicate"],
        },
        EvalCase::BudgetPacking => EvalFixture {
            case,
            task: "small gold",
            graph: EvidenceGraph {
                units: vec![
                    unit_with_source("seed", "fixture:task", "Task seed.", 1.0, 20),
                    unit_with_source("small-gold-a", "fixture:a", "Small gold A.", 0.7, 60),
                    unit_with_source("small-gold-b", "fixture:b", "Small gold B.", 0.7, 60),
                    unit_with_source(
                        "large-distractor",
                        "fixture:large",
                        "Large distracting context.",
                        0.95,
                        120,
                    ),
                ],
                adjacency: vec![vec![1, 2], vec![0], vec![0], vec![]],
            },
            gold: vec!["seed", "small-gold-a", "small-gold-b"],
            bridge: vec![],
            noise: vec!["large-distractor"],
        },
        EvalCase::CrossLinkChain => EvalFixture {
            case,
            task: "linked target evidence",
            graph: EvidenceGraph {
                units: vec![
                    unit_with_source("seed", "fixture:task", "Task seed.", 1.0, 40),
                    unit_with_source("link-a", "fixture:a", "First cross-link hop.", 0.15, 40),
                    unit_with_source("link-b", "fixture:b", "Second cross-link hop.", 0.15, 40),
                    unit_with_source(
                        "target",
                        "fixture:target",
                        "Linked target evidence.",
                        0.9,
                        40,
                    ),
                    unit_with_source(
                        "shortcut-noise",
                        "fixture:noise",
                        "High scoring shortcut noise.",
                        0.85,
                        40,
                    ),
                ],
                adjacency: vec![vec![1, 4], vec![0, 2], vec![1, 3], vec![2], vec![0]],
            },
            gold: vec!["seed", "link-a", "link-b", "target"],
            bridge: vec!["link-a", "link-b"],
            noise: vec!["shortcut-noise"],
        },
    }
}

fn eval_case(
    fixture: EvalFixture,
    method: EvalMethod,
    token_budget: usize,
) -> Result<EvalReport, CropError> {
    let graph = &fixture.graph;
    let crop_pack;
    let selected = match method {
        EvalMethod::Crop => {
            crop_pack = Some(crop_context(
                &graph,
                CropOptions {
                    seed: 0,
                    token_budget,
                },
            )?);
            crop_pack
                .as_ref()
                .expect("crop pack exists")
                .units
                .iter()
                .map(|unit| unit.id.clone())
                .collect::<Vec<_>>()
        }
        EvalMethod::ScoreTopK => {
            crop_pack = None;
            score_top_k_selection(&graph, token_budget)
        }
        EvalMethod::LexicalTopK => {
            crop_pack = None;
            lexical_top_k_selection(&graph, fixture.task, token_budget)
        }
        EvalMethod::HybridTopK => {
            crop_pack = None;
            hybrid_top_k_selection(&graph, fixture.task, token_budget)
        }
    };
    eval_selected_units(&fixture, &selected, token_budget, crop_pack.as_ref()).map(|mut report| {
        report.method = method;
        report
    })
}

fn eval_selected_units(
    fixture: &EvalFixture,
    selected: &[String],
    token_budget: usize,
    crop_pack: Option<&CropPack>,
) -> Result<EvalReport, CropError> {
    let selected_set = selected.iter().cloned().collect::<BTreeSet<_>>();
    let token_estimate = fixture
        .graph
        .units
        .iter()
        .filter(|unit| selected_set.contains(&unit.id))
        .map(|unit| unit.token_estimate)
        .sum::<usize>();
    let gold_recall = recall(&selected_set, &fixture.gold);
    let bridge_recall = recall(&selected_set, &fixture.bridge);
    let source_coverage = source_coverage(&fixture.graph, &selected_set, &fixture.gold);
    let budget_efficiency = if token_estimate == 0 {
        0.0
    } else {
        fixture
            .gold
            .iter()
            .filter(|unit_id| selected_set.contains(**unit_id))
            .count() as f64
            / token_estimate as f64
    };
    let noise_rate = if selected.is_empty() {
        0.0
    } else {
        fixture
            .noise
            .iter()
            .filter(|unit_id| selected_set.contains(**unit_id))
            .count() as f64
            / selected.len() as f64
    };
    let noise_token_share = if token_estimate == 0 {
        0.0
    } else {
        fixture
            .noise
            .iter()
            .filter_map(|unit_id| fixture.graph.units.iter().find(|unit| unit.id == *unit_id))
            .filter(|unit| selected_set.contains(&unit.id))
            .map(|unit| unit.token_estimate)
            .sum::<usize>() as f64
            / token_estimate as f64
    };
    let quality_score = crop_pack
        .as_ref()
        .map(|pack| pack.quality_score)
        .unwrap_or(gold_recall - noise_rate);

    Ok(EvalReport {
        case: fixture.case,
        method: EvalMethod::ScoreTopK,
        token_budget,
        selected_unit_ids: selected.to_vec(),
        token_estimate,
        gold_recall,
        bridge_recall,
        noise_rate,
        source_coverage,
        budget_efficiency,
        noise_token_share,
        quality_score,
    })
}

fn validate_competitor_selection(
    graph: &EvidenceGraph,
    selected_units: &[CompetitorSelectedUnit],
) -> Result<Vec<String>, CropError> {
    let known = graph
        .units
        .iter()
        .map(|unit| unit.id.as_str())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut selected = Vec::new();
    for unit in selected_units {
        if !known.contains(unit.unit_id.as_str()) {
            return Err(CropError::UnknownCompetitorUnit {
                unit_id: unit.unit_id.clone(),
            });
        }
        if !seen.insert(unit.unit_id.clone()) {
            return Err(CropError::DuplicateCompetitorUnit {
                unit_id: unit.unit_id.clone(),
            });
        }
        selected.push(unit.unit_id.clone());
    }
    Ok(selected)
}

fn score_top_k_selection(graph: &EvidenceGraph, token_budget: usize) -> Vec<String> {
    ranked_selection(graph, token_budget, |_, unit| unit.relevance)
}

fn lexical_top_k_selection(graph: &EvidenceGraph, task: &str, token_budget: usize) -> Vec<String> {
    let terms = task_terms(task);
    ranked_selection(graph, token_budget, |_, unit| relevance_score(unit, &terms))
}

fn hybrid_top_k_selection(graph: &EvidenceGraph, task: &str, token_budget: usize) -> Vec<String> {
    let terms = task_terms(task);
    ranked_selection(graph, token_budget, |_, unit| {
        (unit.relevance + relevance_score(unit, &terms)) / 2.0
    })
}

fn ranked_selection<F>(graph: &EvidenceGraph, token_budget: usize, mut score: F) -> Vec<String>
where
    F: FnMut(usize, &EvidenceUnit) -> f64,
{
    let mut candidates = graph.units.iter().enumerate().collect::<Vec<_>>();
    candidates.sort_by(|(left_index, left), (right_index, right)| {
        score(*right_index, right)
            .total_cmp(&score(*left_index, left))
            .then_with(|| left.token_estimate.cmp(&right.token_estimate))
            .then_with(|| left_index.cmp(right_index))
    });

    let mut token_estimate = 0usize;
    let mut selected = Vec::new();
    for (_, unit) in candidates {
        if token_estimate + unit.token_estimate <= token_budget {
            token_estimate += unit.token_estimate;
            selected.push(unit.id.clone());
        }
    }
    selected.sort();
    selected
}

fn recall(selected: &BTreeSet<String>, gold: &[&str]) -> f64 {
    if gold.is_empty() {
        return 1.0;
    }
    gold.iter()
        .filter(|unit_id| selected.contains(**unit_id))
        .count() as f64
        / gold.len() as f64
}

fn source_coverage(graph: &EvidenceGraph, selected: &BTreeSet<String>, gold: &[&str]) -> f64 {
    let gold_sources = gold
        .iter()
        .filter_map(|unit_id| graph.units.iter().find(|unit| unit.id == *unit_id))
        .map(|unit| unit.source_id.as_str())
        .collect::<BTreeSet<_>>();
    if gold_sources.is_empty() {
        return 1.0;
    }
    let selected_gold_sources = gold
        .iter()
        .filter(|unit_id| selected.contains(**unit_id))
        .filter_map(|unit_id| graph.units.iter().find(|unit| unit.id == *unit_id))
        .map(|unit| unit.source_id.as_str())
        .collect::<BTreeSet<_>>();
    selected_gold_sources.len() as f64 / gold_sources.len() as f64
}

fn supported_files(root: &Path) -> Result<Vec<PathBuf>, CropError> {
    let mut files = Vec::new();
    collect_supported_files(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn manifest_root(base_dir: &Path, root: &str) -> PathBuf {
    let root = PathBuf::from(root);
    if root.is_absolute() {
        root
    } else {
        base_dir.join(root)
    }
}

fn collect_supported_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), CropError> {
    let metadata = fs::metadata(path).map_err(|source| CropError::InspectPath {
        path: path.display().to_string(),
        source,
    })?;
    if metadata.is_file() {
        if is_supported_file(path) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }
    if metadata.is_dir() {
        if is_ignored_dir(path) {
            return Ok(());
        }
        let mut children = fs::read_dir(path)
            .map_err(|source| CropError::InspectPath {
                path: path.display().to_string(),
                source,
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| CropError::InspectPath {
                path: path.display().to_string(),
                source,
            })?;
        children.sort_by_key(|entry| entry.path());
        for child in children {
            collect_supported_files(&child.path(), files)?;
        }
    }
    Ok(())
}

fn is_ignored_dir(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".git" | "target")
    )
}

fn is_supported_file(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("md" | "markdown" | "txt" | "json" | "toml" | "yaml" | "yml" | "rs")
    )
}

fn source_id(root: &Path, file: &Path) -> String {
    if root.is_file() {
        return file
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| file.display().to_string());
    }
    let relative = file.strip_prefix(root).unwrap_or(file);
    relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn text_chunks(text: &str) -> Vec<&str> {
    let mut starts = Vec::new();
    let mut cursor = 0;
    let mut chunk_start = None;
    for line in text.split_inclusive('\n') {
        if line.trim().is_empty() {
            if let Some(start) = chunk_start.take() {
                starts.push((start, cursor));
            }
        } else if chunk_start.is_none() {
            chunk_start = Some(cursor);
        }
        cursor += line.len();
    }
    if let Some(start) = chunk_start {
        starts.push((start, text.len()));
    }

    let chunks = starts
        .into_iter()
        .map(|(start, end)| text[start..end].trim())
        .map(str::trim)
        .filter(|chunk| !chunk.is_empty())
        .collect::<Vec<_>>();
    if chunks.is_empty() {
        vec![text.trim()]
    } else {
        chunks
    }
}

fn markdown_links(root: &Path, file: &Path, text: &str) -> Vec<String> {
    if !matches!(
        file.extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref(),
        Some("md" | "markdown")
    ) {
        return Vec::new();
    }

    let mut links = BTreeSet::new();
    for segment in text.split("](").skip(1) {
        let Some((target, _)) = segment.split_once(')') else {
            continue;
        };
        if target.starts_with("http://")
            || target.starts_with("https://")
            || target.starts_with('#')
            || target.is_empty()
        {
            continue;
        }
        let target = target.split('#').next().unwrap_or(target);
        let Some(parent) = file.parent() else {
            continue;
        };
        let candidate = parent.join(target);
        if candidate.exists() && is_supported_file(&candidate) {
            links.insert(source_id(root, &candidate));
        }
    }
    links.into_iter().collect()
}

fn add_undirected_edge(edge_sets: &mut [BTreeSet<usize>], left: usize, right: usize) {
    if left == right {
        return;
    }
    edge_sets[left].insert(right);
    edge_sets[right].insert(left);
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("sha256:{digest:x}")
}

fn elapsed_ms(start: Instant) -> u64 {
    start.elapsed().as_millis().try_into().unwrap_or(u64::MAX)
}

fn estimate_tokens(text: &str) -> usize {
    text.split_whitespace().count().max(1)
}

fn task_terms(task: &str) -> BTreeSet<String> {
    task.split(|character: char| !character.is_ascii_alphanumeric())
        .map(str::to_ascii_lowercase)
        .filter(|term| term.len() >= 3)
        .collect()
}

fn relevance_score(unit: &EvidenceUnit, terms: &BTreeSet<String>) -> f64 {
    if terms.is_empty() {
        return 0.5;
    }
    let haystack = format!("{} {}", unit.id, unit.text).to_ascii_lowercase();
    let matches = terms
        .iter()
        .filter(|term| haystack.contains(term.as_str()))
        .count();
    (matches as f64 / terms.len() as f64).clamp(0.0, 1.0)
}

fn validate_graph(graph: &EvidenceGraph) -> Result<(), CropError> {
    if graph.units.is_empty() {
        return Err(CropError::EmptyGraph);
    }
    if graph.adjacency.len() != graph.units.len() {
        return Err(CropError::AdjacencyLengthMismatch {
            adjacency_len: graph.adjacency.len(),
            unit_count: graph.units.len(),
        });
    }
    for (index, unit) in graph.units.iter().enumerate() {
        if !unit.relevance.is_finite() {
            return Err(CropError::NonFiniteRelevance {
                unit: index,
                score: unit.relevance,
            });
        }
        for &neighbor in &graph.adjacency[index] {
            if neighbor >= graph.units.len() {
                return Err(CropError::NeighborOutOfBounds {
                    unit: index,
                    neighbor,
                    node_count: graph.units.len(),
                });
            }
        }
    }
    Ok(())
}

fn highest_relevance_target(graph: &EvidenceGraph, seed: usize) -> Option<usize> {
    graph
        .units
        .iter()
        .enumerate()
        .filter(|&(index, _)| index != seed)
        .max_by(|(left_index, left), (right_index, right)| {
            left.relevance
                .total_cmp(&right.relevance)
                .then_with(|| right_index.cmp(left_index))
        })
        .map(|(index, _)| index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crops_seed_neighborhood_with_boundary_metric() {
        let graph = EvidenceGraph {
            units: vec![
                unit("seed", 1.0, 80),
                unit("bridge", 0.2, 40),
                unit("target", 0.95, 40),
                unit("noise", 0.9, 40),
            ],
            adjacency: vec![vec![1, 3], vec![0, 2], vec![1], vec![0]],
        };

        let pack = crop_context(
            &graph,
            CropOptions {
                seed: 0,
                token_budget: 170,
            },
        )
        .unwrap();

        assert_eq!(pack.token_estimate, 160);
        assert_eq!(pack.representative_unit_id, "bridge");
        assert_eq!(pack.boundary_cut_edges, 1);
        assert_eq!(pack.boundary_conductance, 1.0);
        assert!((pack.quality_score - 0.573_333_333_333_333_4).abs() < 1e-12);
        assert_eq!(
            pack.units
                .iter()
                .map(|unit| unit.id.as_str())
                .collect::<Vec<_>>(),
            vec!["seed", "bridge", "target"]
        );
        assert_eq!(
            pack.units
                .iter()
                .map(|unit| unit.reason)
                .collect::<Vec<_>>(),
            vec![
                CropReason::Seed,
                CropReason::BridgeConnector,
                CropReason::ConnectorTarget
            ]
        );
    }

    #[test]
    fn rejects_malformed_graph() {
        let graph = EvidenceGraph {
            units: vec![unit("seed", f64::NAN, 100)],
            adjacency: vec![vec![]],
        };

        assert!(matches!(
            crop_context(
                &graph,
                CropOptions {
                    seed: 0,
                    token_budget: 100
                }
            ),
            Err(CropError::NonFiniteRelevance { unit: 0, .. })
        ));
    }

    #[test]
    fn links_all_shared_kernel_dependencies() {
        assert_eq!(shared_kernel_probe(7), ("0.1", SeedPart::Usize(7)));
    }

    #[test]
    fn eval_missing_bridge_shows_crop_beats_score_top_k() {
        let crop = eval_fixture(EvalCase::MissingBridge, EvalMethod::Crop, 160).unwrap();
        let top_k = eval_fixture(EvalCase::MissingBridge, EvalMethod::ScoreTopK, 160).unwrap();

        assert_eq!(
            crop.selected_unit_ids,
            vec!["task-seed", "bridge-note", "target-note"]
        );
        assert_eq!(
            top_k.selected_unit_ids,
            vec!["stale-noise", "target-note", "task-seed"]
        );
        assert_eq!(crop.gold_recall, 1.0);
        assert_eq!(crop.bridge_recall, 1.0);
        assert_eq!(crop.noise_rate, 0.0);
        assert_eq!(crop.source_coverage, 1.0);
        assert!((crop.budget_efficiency - (3.0 / 160.0)).abs() < 1e-12);
        assert_eq!(crop.noise_token_share, 0.0);
        assert!(top_k.gold_recall < crop.gold_recall);
        assert_eq!(top_k.bridge_recall, 0.0);
        assert!(top_k.noise_rate > 0.0);
        assert!(top_k.source_coverage < crop.source_coverage);
        assert!(top_k.noise_token_share > crop.noise_token_share);
    }

    #[test]
    fn eval_expanded_fixtures_show_distinct_crop_wins() {
        for case in [
            EvalCase::NoisyPeriphery,
            EvalCase::StaleDuplicate,
            EvalCase::BudgetPacking,
            EvalCase::CrossLinkChain,
        ] {
            let crop = eval_fixture(case, EvalMethod::Crop, 160).unwrap();
            let top_k = eval_fixture(case, EvalMethod::ScoreTopK, 160).unwrap();

            assert_eq!(crop.gold_recall, 1.0, "{case:?}");
            assert_eq!(crop.noise_rate, 0.0, "{case:?}");
            assert_eq!(crop.source_coverage, 1.0, "{case:?}");
            assert_eq!(crop.noise_token_share, 0.0, "{case:?}");
            assert!(
                top_k.gold_recall < crop.gold_recall || top_k.noise_rate > crop.noise_rate,
                "{case:?}"
            );
            assert!(top_k.noise_rate > crop.noise_rate, "{case:?}");
        }
    }

    #[test]
    fn eval_cross_link_chain_tracks_bridge_recall() {
        let crop = eval_fixture(EvalCase::CrossLinkChain, EvalMethod::Crop, 160).unwrap();
        let top_k = eval_fixture(EvalCase::CrossLinkChain, EvalMethod::ScoreTopK, 160).unwrap();

        assert_eq!(
            crop.selected_unit_ids,
            vec!["seed", "link-a", "link-b", "target"]
        );
        assert_eq!(crop.bridge_recall, 1.0);
        assert!(top_k.bridge_recall < crop.bridge_recall);
    }

    #[test]
    fn lexical_and_hybrid_baselines_can_win_when_graph_is_not_needed() {
        let lexical = eval_fixture(EvalCase::StaleDuplicate, EvalMethod::LexicalTopK, 110).unwrap();
        let hybrid = eval_fixture(EvalCase::StaleDuplicate, EvalMethod::HybridTopK, 110).unwrap();
        let score = eval_fixture(EvalCase::StaleDuplicate, EvalMethod::ScoreTopK, 110).unwrap();

        assert_eq!(lexical.selected_unit_ids, vec!["current-canonical", "seed"]);
        assert_eq!(hybrid.selected_unit_ids, vec!["current-canonical", "seed"]);
        assert_eq!(lexical.gold_recall, 1.0);
        assert_eq!(hybrid.gold_recall, 1.0);
        assert_eq!(lexical.noise_rate, 0.0);
        assert_eq!(hybrid.noise_rate, 0.0);
        assert!(score.noise_rate > lexical.noise_rate);
    }

    #[test]
    fn lexical_and_hybrid_are_budgeted_and_deterministic() {
        let lexical = eval_fixture(EvalCase::MissingBridge, EvalMethod::LexicalTopK, 160).unwrap();
        let hybrid = eval_fixture(EvalCase::MissingBridge, EvalMethod::HybridTopK, 160).unwrap();

        assert!(lexical.token_estimate <= 160);
        assert!(hybrid.token_estimate <= 160);
        assert_eq!(
            lexical.selected_unit_ids,
            eval_fixture(EvalCase::MissingBridge, EvalMethod::LexicalTopK, 160)
                .unwrap()
                .selected_unit_ids
        );
        assert_eq!(
            hybrid.selected_unit_ids,
            eval_fixture(EvalCase::MissingBridge, EvalMethod::HybridTopK, 160)
                .unwrap()
                .selected_unit_ids
        );
    }

    #[test]
    fn eval_suite_has_stable_case_and_method_ordering() {
        let suite = eval_suite(160).unwrap();

        assert_eq!(suite.cases, all_eval_cases());
        assert_eq!(suite.methods, all_eval_methods());
        assert_eq!(suite.rows.len(), suite.cases.len() * suite.methods.len());
        assert_eq!(suite.aggregates.len(), suite.methods.len());
        assert_eq!(suite.rows[0].case, EvalCase::MissingBridge);
        assert_eq!(suite.rows[0].method, EvalMethod::Crop);
        assert_eq!(suite.rows[1].method, EvalMethod::ScoreTopK);
        assert_eq!(suite.aggregates[0].method, EvalMethod::Crop);
        assert_eq!(suite.aggregates[0].case_count, suite.cases.len());
        assert!(suite.aggregates[0].mean_gold_recall > suite.aggregates[1].mean_gold_recall);
        assert!(suite.aggregates[0].mean_noise_rate < suite.aggregates[1].mean_noise_rate);
        assert!(
            suite.aggregates[0].mean_source_coverage > suite.aggregates[1].mean_source_coverage
        );
        assert!(
            suite.aggregates[0].mean_noise_token_share < suite.aggregates[1].mean_noise_token_share
        );
        assert!(suite.aggregates[0].mean_budget_efficiency > 0.0);
    }

    #[test]
    fn competitor_import_scores_saved_results_with_same_metrics() {
        let input = CompetitorEvalInput {
            schema_version: COMPETITOR_RESULTS_SCHEMA.to_string(),
            case: EvalCase::MissingBridge,
            token_budget: 160,
            method: competitor_method("qdrant-fixture"),
            selected_units: vec![
                competitor_unit("task-seed", 0.99, 80),
                competitor_unit("target-note", 0.98, 40),
                competitor_unit("stale-noise", 0.97, 40),
            ],
        };

        let report = eval_competitor_results(input).unwrap();

        assert_eq!(report.schema_version, "crop.competitor-results.v1");
        assert_eq!(report.method.method_id, "qdrant-fixture");
        assert_eq!(
            report.selected_unit_ids,
            vec!["task-seed", "target-note", "stale-noise"]
        );
        assert_eq!(report.token_estimate, 160);
        assert_eq!(report.reported_token_estimate, Some(160));
        assert_eq!(report.gold_recall, 2.0 / 3.0);
        assert_eq!(report.bridge_recall, 0.0);
        assert_eq!(report.noise_rate, 1.0 / 3.0);
        assert_eq!(report.source_coverage, 2.0 / 3.0);
        assert_eq!(report.budget_efficiency, 2.0 / 160.0);
        assert_eq!(report.noise_token_share, 40.0 / 160.0);
        assert_eq!(report.quality_score, report.gold_recall - report.noise_rate);
    }

    #[test]
    fn competitor_json_import_rejects_unknown_and_duplicate_units() {
        let unknown = CompetitorEvalInput {
            schema_version: COMPETITOR_RESULTS_SCHEMA.to_string(),
            case: EvalCase::MissingBridge,
            token_budget: 160,
            method: competitor_method("bad-import"),
            selected_units: vec![competitor_unit("missing-unit", 0.5, 40)],
        };
        assert!(matches!(
            eval_competitor_results(unknown),
            Err(CropError::UnknownCompetitorUnit { .. })
        ));

        let unsupported = CompetitorEvalInput {
            schema_version: "crop.competitor-results.v0".to_string(),
            case: EvalCase::MissingBridge,
            token_budget: 160,
            method: competitor_method("bad-schema"),
            selected_units: vec![competitor_unit("task-seed", 0.5, 80)],
        };
        assert!(matches!(
            eval_competitor_results(unsupported),
            Err(CropError::UnsupportedCompetitorSchema { .. })
        ));

        let duplicate = r#"{
          "schema_version": "crop.competitor-results.v1",
          "case": "missing-bridge",
          "token_budget": 160,
          "method": {
            "method_id": "duplicate-import",
            "competitor_class": "vector",
            "implementation": "offline-fixture",
            "version": null,
            "notes": null
          },
          "selected_units": [
            {"unit_id": "task-seed", "score": 0.9, "token_estimate": 80},
            {"unit_id": "task-seed", "score": 0.8, "token_estimate": 80}
          ]
        }"#;
        assert!(matches!(
            eval_competitor_json(duplicate),
            Err(CropError::DuplicateCompetitorUnit { .. })
        ));
    }

    #[test]
    fn checked_competitor_examples_are_scoreable() {
        let examples = [
            include_str!("../../../examples/competitors/qdrant-vector.missing-bridge.json"),
            include_str!("../../../examples/competitors/tantivy-lexical.stale-duplicate.json"),
            include_str!("../../../examples/competitors/hybrid-reranker.budget-packing.json"),
            include_str!("../../../examples/competitors/llamaindex-rag.noisy-periphery.json"),
            include_str!("../../../examples/competitors/graphrag-neo4j.cross-link-chain.json"),
        ];

        for example in examples {
            let report = eval_competitor_json(example).unwrap();

            assert_eq!(report.schema_version, COMPETITOR_RESULTS_SCHEMA);
            assert!(!report.method.method_id.is_empty());
            assert!(!report.selected_unit_ids.is_empty());
            assert!(report.token_estimate <= report.token_budget);
            assert!(report.quality_score.is_finite());
        }
    }

    #[test]
    fn ingests_markdown_links_and_scores_task_terms() {
        let root = unique_temp_dir("crop-ingest");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs").join("index.md"),
            "Task overview links to [target](target.md).\n\nBridge context.",
        )
        .unwrap();
        fs::write(
            root.join("docs").join("target.md"),
            "Target evidence explains provenance replay.",
        )
        .unwrap();

        let mut ingested = ingest_path(&root).unwrap();
        score_graph_for_task(&mut ingested.graph, "provenance replay");

        assert_eq!(ingested.report.source_count, 2);
        assert_eq!(ingested.report.unit_count, 3);
        assert!(ingested.report.edge_count >= 2);
        assert!(ingested
            .graph
            .units
            .iter()
            .any(|unit| unit.content_hash.starts_with("sha256:")));
        assert!(ingested.graph.units.iter().any(|unit| unit.relevance > 0.0));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn prefix_cache_key_is_stable_when_only_task_changes() {
        let root = unique_temp_dir("crop-prefix");
        fs::write(
            root.with_extension("md"),
            "# Prefix Cache\n\nStable source body for repeated crops.",
        )
        .unwrap();
        let path = root.with_extension("md");

        let first = prefix_cached_pack_for_task(&path, "explain prefix cache");
        let second = prefix_cached_pack_for_task(&path, "summarize stable corpus");

        assert_eq!(first.prefix_hash, second.prefix_hash);
        assert_eq!(first.cache_key, second.cache_key);
        assert_eq!(first.prefix, second.prefix);
        assert_ne!(first.task_tail.task, second.task_tail.task);
        assert!(first.cache_key.starts_with("crop-prefix:generic:"));
        assert_eq!(first.prefix.sources.len(), 1);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn prefix_cache_key_changes_when_source_content_changes() {
        let path = unique_temp_dir("crop-prefix-change").with_extension("md");
        fs::write(&path, "Original source.").unwrap();
        let original = prefix_cached_pack_for_task(&path, "explain source");

        fs::write(&path, "Changed source.").unwrap();
        let changed = prefix_cached_pack_for_task(&path, "explain source");

        assert_ne!(original.prefix_hash, changed.prefix_hash);
        assert_ne!(original.cache_key, changed.cache_key);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn ingest_skips_git_and_target_directories() {
        let root = unique_temp_dir("crop-ignore");
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::create_dir_all(root.join("target")).unwrap();
        fs::write(root.join("README.md"), "Visible context.").unwrap();
        fs::write(root.join(".git").join("config.toml"), "hidden = true").unwrap();
        fs::write(
            root.join("target").join("generated.rs"),
            "fn generated() {}",
        )
        .unwrap();

        let ingested = ingest_path(&root).unwrap();

        assert_eq!(ingested.report.source_count, 1);
        assert_eq!(ingested.graph.units[0].source_id, "README.md");
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bench_pack_reports_stage_timings_and_pack_summary() {
        let root = unique_temp_dir("crop-bench");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs").join("index.md"),
            "Benchmark overview links to [target](target.md).\n\nTask seed context.",
        )
        .unwrap();
        fs::write(
            root.join("docs").join("target.md"),
            "Target benchmark evidence explains graph crops.",
        )
        .unwrap();

        let report = bench_pack(
            &root,
            "benchmark graph crops",
            CropOptions {
                seed: 0,
                token_budget: 160,
            },
        )
        .unwrap();

        assert_eq!(report.schema_version, "crop.bench.v1");
        assert_eq!(report.source_count, 2);
        assert_eq!(report.unit_count, 3);
        assert!(report.edge_count >= 2);
        assert!(report.selected_unit_count > 0);
        assert!(report.selected_token_estimate <= 160);
        assert!(report.quality_score.is_finite());
        assert!(report.timings.total_ms >= report.timings.crop_ms);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bench_manifest_runs_named_views_relative_to_manifest_dir() {
        let root = unique_temp_dir("crop-bench-manifest");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs").join("index.md"),
            "Benchmark manifest links to [target](target.md).\n\nTask seed context.",
        )
        .unwrap();
        fs::write(
            root.join("docs").join("target.md"),
            "Target benchmark evidence explains reusable views.",
        )
        .unwrap();

        let manifest = r#"{
          "schema_version": "crop.bench-manifest.v1",
          "cases": [
            {
              "id": "docs-view",
              "root": "docs",
              "task": "benchmark reusable views",
              "token_budget": 160,
              "seed": 0,
              "view": "docs-only"
            },
            {
              "id": "root-view",
              "root": ".",
              "task": "benchmark graph crops",
              "token_budget": 160,
              "seed": 0,
              "view": "whole-root"
            }
          ]
        }"#;

        let report = bench_manifest_json(manifest, &root).unwrap();

        assert_eq!(report.schema_version, "crop.bench-report.v1");
        assert_eq!(report.case_count, 2);
        assert_eq!(report.aggregate.method, "crop");
        assert_eq!(report.aggregate.case_count, 2);
        assert_eq!(report.cases[0].id, "docs-view");
        assert_eq!(report.cases[0].view.as_deref(), Some("docs-only"));
        assert_eq!(report.cases[0].report.source_count, 2);
        assert!(report.cases[1].report.unit_count >= report.cases[0].report.unit_count);
        assert_eq!(
            report.aggregate.total_source_count,
            report
                .cases
                .iter()
                .map(|case| case.report.source_count)
                .sum::<usize>()
        );
        assert_eq!(
            report.aggregate.total_selected_token_estimate,
            report
                .cases
                .iter()
                .map(|case| case.report.selected_token_estimate)
                .sum::<usize>()
        );
        assert!(
            report.aggregate.mean_quality_score
                <= report
                    .cases
                    .iter()
                    .map(|case| case.report.quality_score)
                    .fold(f64::NEG_INFINITY, f64::max)
        );
        assert!(
            report.aggregate.max_total_ms
                >= report
                    .cases
                    .iter()
                    .map(|case| case.report.timings.total_ms)
                    .max()
                    .unwrap()
        );
        assert!(report
            .cases
            .iter()
            .all(|case| case.report.selected_token_estimate <= 160));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn bench_manifest_rejects_bad_schema_and_empty_cases() {
        let root = unique_temp_dir("crop-bench-manifest-bad");
        fs::create_dir_all(&root).unwrap();

        let bad_schema = BenchManifest {
            schema_version: "crop.bench-manifest.v0".to_string(),
            cases: vec![BenchManifestCase {
                id: "bad".to_string(),
                root: ".".to_string(),
                task: "bad".to_string(),
                token_budget: 160,
                seed: 0,
                view: None,
            }],
        };
        assert!(matches!(
            bench_manifest(bad_schema, &root),
            Err(CropError::UnsupportedBenchManifestSchema { .. })
        ));

        let empty = BenchManifest {
            schema_version: BENCH_MANIFEST_SCHEMA.to_string(),
            cases: Vec::new(),
        };
        assert!(matches!(
            bench_manifest(empty, &root),
            Err(CropError::EmptyBenchManifest)
        ));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_view_json_emits_named_context_pack() {
        let root = unique_temp_dir("crop-view");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs").join("index.md"),
            "View recipe links to [target](target.md).\n\nTask seed context.",
        )
        .unwrap();
        fs::write(
            root.join("docs").join("target.md"),
            "Target view evidence explains persisted crops.",
        )
        .unwrap();

        let view = r#"{
          "schema_version": "crop.view.v1",
          "name": "docs-view",
          "root": "docs",
          "task": "persisted view crops",
          "token_budget": 160,
          "seed": 0
        }"#;

        let run = run_view_json(view, &root).unwrap();

        assert_eq!(run.schema_version, "crop.view-run.v1");
        assert_eq!(run.name, "docs-view");
        assert_eq!(run.root, "docs");
        assert_eq!(run.token_budget, 160);
        assert!(!run.pack.units.is_empty());
        assert!(run.pack.token_estimate <= 160);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn run_view_rejects_bad_schema_and_zero_budget() {
        let root = unique_temp_dir("crop-view-bad");
        fs::create_dir_all(&root).unwrap();

        let bad_schema = CropView {
            schema_version: "crop.view.v0".to_string(),
            name: "bad".to_string(),
            root: ".".to_string(),
            task: "bad".to_string(),
            token_budget: 160,
            seed: 0,
        };
        assert!(matches!(
            run_view(bad_schema, &root),
            Err(CropError::UnsupportedViewSchema { .. })
        ));

        let zero_budget = CropView {
            schema_version: CROP_VIEW_SCHEMA.to_string(),
            name: "zero".to_string(),
            root: ".".to_string(),
            task: "bad".to_string(),
            token_budget: 0,
            seed: 0,
        };
        assert!(matches!(
            run_view(zero_budget, &root),
            Err(CropError::NonPositiveBudget)
        ));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn list_views_reads_sorted_json_view_store() {
        let root = unique_temp_dir("crop-view-list");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("ignore.txt"), "not a view").unwrap();
        fs::write(
            root.join("b-view.json"),
            r#"{
              "schema_version": "crop.view.v1",
              "name": "beta",
              "root": "docs",
              "task": "beta task",
              "token_budget": 200,
              "seed": 1
            }"#,
        )
        .unwrap();
        fs::write(
            root.join("a-view.json"),
            r#"{
              "schema_version": "crop.view.v1",
              "name": "alpha",
              "root": ".",
              "task": "alpha task",
              "token_budget": 100,
              "seed": 0
            }"#,
        )
        .unwrap();

        let list = list_views(&root).unwrap();

        assert_eq!(list.schema_version, "crop.view-list.v1");
        assert_eq!(list.view_count, 2);
        assert_eq!(list.views[0].file, "a-view.json");
        assert_eq!(list.views[0].name, "alpha");
        assert_eq!(list.views[1].file, "b-view.json");
        assert_eq!(list.views[1].token_budget, 200);

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ingest_rejects_missing_or_unsupported_root() {
        let root = unique_temp_dir("crop-empty");
        assert!(matches!(
            ingest_path(root.join("missing")),
            Err(CropError::RootDoesNotExist { .. })
        ));

        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("image.bin"), [0, 159, 146, 150]).unwrap();
        assert!(matches!(
            ingest_path(&root),
            Err(CropError::NoSupportedFiles { .. })
        ));
        fs::remove_dir_all(root).unwrap();
    }

    fn unit(id: &str, relevance: f64, token_estimate: usize) -> EvidenceUnit {
        unit_with_source(
            id,
            &format!("source:{id}"),
            &format!("fixture text for {id}"),
            relevance,
            token_estimate,
        )
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "{prefix}-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn prefix_cached_pack_for_task(path: &Path, task: &str) -> PrefixCachedPack {
        let mut ingested = ingest_path(path).unwrap();
        score_graph_for_task(&mut ingested.graph, task);
        let options = CropOptions {
            seed: 0,
            token_budget: 160,
        };
        let pack = crop_context(&ingested.graph, options.clone()).unwrap();
        prefix_cached_pack(&ingested, pack, task, options, PrefixCacheProfile::Generic).unwrap()
    }

    fn competitor_method(method_id: &str) -> CompetitorMethodMetadata {
        CompetitorMethodMetadata {
            method_id: method_id.to_string(),
            competitor_class: CompetitorClass::Vector,
            implementation: "offline-fixture".to_string(),
            version: Some("test".to_string()),
            notes: None,
        }
    }

    fn competitor_unit(unit_id: &str, score: f64, token_estimate: usize) -> CompetitorSelectedUnit {
        CompetitorSelectedUnit {
            unit_id: unit_id.to_string(),
            score: Some(score),
            token_estimate: Some(token_estimate),
        }
    }
}

fn unit_with_source(
    id: &str,
    source_id: &str,
    text: &str,
    relevance: f64,
    token_estimate: usize,
) -> EvidenceUnit {
    EvidenceUnit {
        id: id.to_string(),
        source_id: source_id.to_string(),
        content_hash: sha256_hex(text.as_bytes()),
        text: text.to_string(),
        relevance,
        token_estimate,
    }
}
