use crate::CropError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MdloomArtifactManifest {
    pub schema_version: String,
    pub generated_by: String,
    pub config_root: PathBuf,
    pub generated_at_ms: u64,
    pub artifacts: Vec<MdloomArtifactRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MdloomArtifactRecord {
    pub source_path: PathBuf,
    pub output_path: PathBuf,
    pub target: String,
    pub status: String,
    pub directives_resolved: usize,
    pub from_cache: bool,
    pub diagnostics: Vec<MdloomArtifactDiagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MdloomArtifactDiagnostic {
    pub code: String,
    pub severity: String,
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MdloomArtifactReportEntry {
    pub source: String,
    pub output: String,
    pub target: String,
    pub status: String,
    pub output_exists: bool,
    pub stale: bool,
    pub directives_resolved: usize,
    pub from_cache: bool,
    pub diagnostic_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MdloomArtifactReport {
    pub schema_version: String,
    pub manifest: String,
    pub generated_by: String,
    pub artifact_count: usize,
    pub ok_count: usize,
    pub missing_output_count: usize,
    pub stale_count: usize,
    pub diagnostic_count: usize,
    pub artifacts: Vec<MdloomArtifactReportEntry>,
}

pub fn mdloom_artifacts_for_root(
    root: impl AsRef<Path>,
) -> Result<MdloomArtifactReport, CropError> {
    mdloom_artifacts(root.as_ref().join(".mdloom").join("artifacts.json"))
}

pub fn mdloom_artifacts(manifest: impl AsRef<Path>) -> Result<MdloomArtifactReport, CropError> {
    let manifest = manifest.as_ref();
    if !manifest.exists() {
        return Err(CropError::RootDoesNotExist {
            path: manifest.display().to_string(),
        });
    }
    let json = fs::read_to_string(manifest).map_err(|source| CropError::ReadFile {
        path: manifest.display().to_string(),
        source,
    })?;
    mdloom_artifacts_json(&json, manifest)
}

pub fn render_mdloom_artifact_markdown(report: &MdloomArtifactReport) -> String {
    let mut markdown = String::new();
    markdown.push_str("# MDLOOM Artifact Health\n\n");
    markdown.push_str(&format!("Manifest: `{}`\n\n", report.manifest));
    markdown.push_str(&format!(
        "Artifacts: **{}** | OK: **{}** | Missing: **{}** | Stale: **{}** | Diagnostics: **{}**\n\n",
        report.artifact_count,
        report.ok_count,
        report.missing_output_count,
        report.stale_count,
        report.diagnostic_count,
    ));
    markdown.push_str("| Source | Output | Target | Status | Exists | Stale | Cache | Directives | Diagnostics |\n");
    markdown.push_str("|--------|--------|--------|--------|--------|-------|-------|-----------:|------------:|\n");
    for artifact in &report.artifacts {
        markdown.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | {} | {} | {} | {} | {} |\n",
            escape_markdown_cell(&artifact.source),
            escape_markdown_cell(&artifact.output),
            escape_markdown_cell(&artifact.target),
            escape_markdown_cell(&artifact.status),
            yes_no(artifact.output_exists),
            yes_no(artifact.stale),
            yes_no(artifact.from_cache),
            artifact.directives_resolved,
            artifact.diagnostic_count,
        ));
    }
    markdown
}

pub fn mdloom_artifacts_json(
    json: &str,
    manifest_path: impl AsRef<Path>,
) -> Result<MdloomArtifactReport, CropError> {
    let manifest: MdloomArtifactManifest =
        serde_json::from_str(json).map_err(CropError::ArtifactManifestImport)?;
    let manifest_path = manifest_path.as_ref();
    let base = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let root = if manifest.config_root.is_absolute() {
        manifest.config_root.clone()
    } else {
        base.join(&manifest.config_root)
    };

    let mut artifacts = Vec::new();
    for artifact in manifest.artifacts {
        let source_path = resolve_artifact_path(&root, &artifact.source_path);
        let output_path = resolve_artifact_path(&root, &artifact.output_path);
        let output_exists = output_path.exists();
        let stale = output_exists
            && source_path.exists()
            && file_mtime_ms(&source_path)? > file_mtime_ms(&output_path)?;
        artifacts.push(MdloomArtifactReportEntry {
            source: display_path(&artifact.source_path),
            output: display_path(&artifact.output_path),
            target: artifact.target,
            status: artifact.status,
            output_exists,
            stale,
            directives_resolved: artifact.directives_resolved,
            from_cache: artifact.from_cache,
            diagnostic_count: artifact.diagnostics.len(),
        });
    }

    let missing_output_count = artifacts
        .iter()
        .filter(|artifact| !artifact.output_exists)
        .count();
    let stale_count = artifacts.iter().filter(|artifact| artifact.stale).count();
    let diagnostic_count = artifacts
        .iter()
        .map(|artifact| artifact.diagnostic_count)
        .sum();
    let ok_count = artifacts
        .len()
        .saturating_sub(missing_output_count + stale_count);
    Ok(MdloomArtifactReport {
        schema_version: "crop.mdloom-artifacts.v1".to_string(),
        manifest: display_path(manifest_path),
        generated_by: manifest.generated_by,
        artifact_count: artifacts.len(),
        ok_count,
        missing_output_count,
        stale_count,
        diagnostic_count,
        artifacts,
    })
}

fn resolve_artifact_path(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn file_mtime_ms(path: &Path) -> Result<u64, CropError> {
    let modified = fs::metadata(path)
        .map_err(|source| CropError::InspectPath {
            path: path.display().to_string(),
            source,
        })?
        .modified()
        .map_err(|source| CropError::InspectPath {
            path: path.display().to_string(),
            source,
        })?;
    Ok(modified
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64)
}

fn display_path(path: &Path) -> String {
    let rendered = path.display().to_string();
    rendered
        .strip_prefix(r"\\?\")
        .unwrap_or(&rendered)
        .to_string()
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

fn escape_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}
