use anyhow::{bail, Result};
use crop_core::{
    markdown_backlinks, markdown_backlinks_for_view_json, markdown_catalog,
    markdown_catalog_for_view_json, markdown_frontmatter_inventory,
    markdown_frontmatter_inventory_for_view_json, markdown_heading_inventory,
    markdown_heading_inventory_for_view_json, markdown_link_audit,
    markdown_link_audit_for_view_json, proof_artifacts_for_root, render_markdown_backlinks,
    render_markdown_frontmatter_inventory, render_markdown_heading_inventory,
    render_markdown_link_audit, render_proof_artifact_markdown, CropView, IngestOptions,
};
use std::fs;
use std::path::PathBuf;

pub fn run(
    root: PathBuf,
    docs_root: Option<PathBuf>,
    view: Option<PathBuf>,
    title: String,
    extensions: Vec<String>,
    exclude_dirs: Vec<String>,
    no_artifacts: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    if docs_root.is_some() && view.is_some() {
        bail!("--docs-root and --view cannot be combined");
    }

    let docs = if let Some(view) = view {
        status_docs_for_view(view, extensions, exclude_dirs)?
    } else {
        status_docs_for_root(
            docs_root.unwrap_or_else(|| root.clone()),
            extensions,
            exclude_dirs,
        )?
    };

    let mut markdown = String::new();
    markdown.push_str(&format!("# {}\n\n", title));
    markdown.push_str(&format!("PROOF root: `{}`\n\n", root.display()));
    markdown.push_str(&format!("Docs input: `{}`\n\n", docs.label));
    markdown.push_str("## Health summary\n\n");
    markdown.push_str("| Surface | Count | Issues |\n");
    markdown.push_str("|---------|------:|-------:|\n");
    markdown.push_str(&format!(
        "| Catalog sources | {} | 0 |\n",
        docs.backlinks.source_count
    ));
    markdown.push_str(&format!(
        "| Links | {} | {} |\n",
        docs.links.link_count, docs.links.broken_count
    ));
    markdown.push_str(&format!(
        "| Orphan pages | {} | {} |\n",
        docs.backlinks.source_count, docs.backlinks.orphan_count
    ));
    markdown.push_str(&format!(
        "| Frontmatter pages | {} | 0 |\n",
        docs.frontmatter.page_count
    ));
    markdown.push_str(&format!(
        "| Headings | {} | {} |\n",
        docs.headings.heading_count, docs.headings.duplicate_anchor_count
    ));

    if !no_artifacts {
        let artifacts = proof_artifacts_for_root(&root)?;
        markdown.push_str(&format!(
            "| Artifacts | {} | {} |\n",
            artifacts.artifact_count,
            artifacts.missing_output_count + artifacts.stale_count + artifacts.diagnostic_count
        ));
        markdown.push_str("\n---\n\n");
        markdown.push_str(&render_proof_artifact_markdown(&artifacts));
    }

    markdown.push_str("\n---\n\n");
    markdown.push_str(&docs.catalog);
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_link_audit(&docs.links));
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_backlinks(&docs.backlinks));
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_frontmatter_inventory(&docs.frontmatter));
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_heading_inventory(&docs.headings));

    if let Some(output) = output {
        fs::write(output, markdown)?;
    } else {
        println!("{markdown}");
    }
    Ok(())
}

struct StatusDocs {
    label: String,
    catalog: String,
    links: crop_core::MarkdownLinkAudit,
    backlinks: crop_core::MarkdownBacklinkReport,
    frontmatter: crop_core::MarkdownFrontmatterInventory,
    headings: crop_core::MarkdownHeadingInventory,
}

fn status_docs_for_root(
    docs_root: PathBuf,
    extensions: Vec<String>,
    exclude_dirs: Vec<String>,
) -> Result<StatusDocs> {
    let options = IngestOptions {
        include_extensions: if extensions.is_empty() {
            vec!["md".to_string()]
        } else {
            extensions
        },
        exclude_dirs,
        frontmatter_query: None,
    };

    Ok(StatusDocs {
        label: docs_root.display().to_string(),
        catalog: markdown_catalog(&docs_root, "Document Catalog", &options)?,
        links: markdown_link_audit(&docs_root, &options)?,
        backlinks: markdown_backlinks(&docs_root, &options)?,
        frontmatter: markdown_frontmatter_inventory(&docs_root, &options)?,
        headings: markdown_heading_inventory(&docs_root, &options)?,
    })
}

fn status_docs_for_view(
    view: PathBuf,
    extensions: Vec<String>,
    exclude_dirs: Vec<String>,
) -> Result<StatusDocs> {
    let input = fs::read_to_string(&view)?;
    let base_dir = view.parent().unwrap_or_else(|| std::path::Path::new("."));
    let mut recipe: CropView = serde_json::from_str(&input)?;
    if !extensions.is_empty() {
        recipe.include_extensions = extensions;
    }
    recipe.exclude_dirs.extend(exclude_dirs);
    let view_json = serde_json::to_string(&recipe)?;

    Ok(StatusDocs {
        label: view.display().to_string(),
        catalog: markdown_catalog_for_view_json(&view_json, base_dir)?,
        links: markdown_link_audit_for_view_json(&view_json, base_dir)?,
        backlinks: markdown_backlinks_for_view_json(&view_json, base_dir)?,
        frontmatter: markdown_frontmatter_inventory_for_view_json(&view_json, base_dir)?,
        headings: markdown_heading_inventory_for_view_json(&view_json, base_dir)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn proof_status_renders_combined_markdown_page() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("crop-proof-status.md");

        run(
            fixture.clone(),
            Some(fixture.join("src")),
            None,
            "Fixture Status".to_string(),
            vec!["md".to_string()],
            Vec::new(),
            false,
            Some(output.clone()),
        )
        .unwrap();

        let markdown = fs::read_to_string(&output).unwrap();
        assert!(markdown.contains("# Fixture Status"));
        assert!(markdown.contains("## Health summary"));
        assert!(markdown.contains("# PROOF Artifact Health"));
        assert!(markdown.contains("# Document Catalog"));
        assert!(markdown.contains("# Markdown Link Audit"));
        assert!(markdown.contains("# Markdown Headings"));

        fs::remove_file(output).unwrap();
    }

    #[test]
    fn proof_status_accepts_named_view_docs_input() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("crop-proof-view-status.md");

        run(
            fixture.clone(),
            None,
            Some(fixture.join("proof-ready-view.json")),
            "View Status".to_string(),
            Vec::new(),
            Vec::new(),
            false,
            Some(output.clone()),
        )
        .unwrap();

        let markdown = fs::read_to_string(&output).unwrap();
        assert!(markdown.contains("# View Status"));
        assert!(markdown.contains("Docs input: `"));
        assert!(markdown.contains("# proof-fixture-ready"));
        assert!(markdown.contains("guide.source.md"));
        assert!(!markdown.contains("draft.source.md"));

        fs::remove_file(output).unwrap();
    }

    fn unique_temp_file(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{nanos}-{name}"))
    }
}
