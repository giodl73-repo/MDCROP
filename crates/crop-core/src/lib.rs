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
use thiserror::Error;

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
pub enum EvalCase {
    MissingBridge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EvalMethod {
    Crop,
    ScoreTopK,
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
    pub quality_score: f64,
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
    match case {
        EvalCase::MissingBridge => eval_missing_bridge(method, token_budget),
    }
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

fn eval_missing_bridge(method: EvalMethod, token_budget: usize) -> Result<EvalReport, CropError> {
    let graph = fixture_graph();
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
    };
    let selected_set = selected.iter().cloned().collect::<BTreeSet<_>>();
    let token_estimate = graph
        .units
        .iter()
        .filter(|unit| selected_set.contains(&unit.id))
        .map(|unit| unit.token_estimate)
        .sum::<usize>();
    let gold = ["task-seed", "bridge-note", "target-note"];
    let bridge = ["bridge-note"];
    let noise = ["stale-noise"];
    let gold_recall = recall(&selected_set, &gold);
    let bridge_recall = recall(&selected_set, &bridge);
    let noise_rate = if selected.is_empty() {
        0.0
    } else {
        noise
            .iter()
            .filter(|unit_id| selected_set.contains(**unit_id))
            .count() as f64
            / selected.len() as f64
    };
    let quality_score = crop_pack
        .as_ref()
        .map(|pack| pack.quality_score)
        .unwrap_or(gold_recall - noise_rate);

    Ok(EvalReport {
        case: EvalCase::MissingBridge,
        method,
        token_budget,
        selected_unit_ids: selected,
        token_estimate,
        gold_recall,
        bridge_recall,
        noise_rate,
        quality_score,
    })
}

fn score_top_k_selection(graph: &EvidenceGraph, token_budget: usize) -> Vec<String> {
    let mut candidates = graph.units.iter().enumerate().collect::<Vec<_>>();
    candidates.sort_by(|(left_index, left), (right_index, right)| {
        right
            .relevance
            .total_cmp(&left.relevance)
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

fn supported_files(root: &Path) -> Result<Vec<PathBuf>, CropError> {
    let mut files = Vec::new();
    collect_supported_files(root, &mut files)?;
    files.sort();
    Ok(files)
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
        assert!(top_k.gold_recall < crop.gold_recall);
        assert_eq!(top_k.bridge_recall, 0.0);
        assert!(top_k.noise_rate > 0.0);
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
