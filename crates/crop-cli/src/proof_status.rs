use anyhow::Result;
use crop_core::{
    markdown_backlinks, markdown_catalog, markdown_frontmatter_inventory,
    markdown_heading_inventory, markdown_link_audit, proof_artifacts_for_root,
    render_markdown_backlinks, render_markdown_frontmatter_inventory,
    render_markdown_heading_inventory, render_markdown_link_audit, render_proof_artifact_markdown,
    IngestOptions,
};
use std::fs;
use std::path::PathBuf;

pub fn run(
    root: PathBuf,
    docs_root: Option<PathBuf>,
    title: String,
    extensions: Vec<String>,
    exclude_dirs: Vec<String>,
    no_artifacts: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    let docs_root = docs_root.unwrap_or_else(|| root.clone());
    let options = IngestOptions {
        include_extensions: if extensions.is_empty() {
            vec!["md".to_string()]
        } else {
            extensions
        },
        exclude_dirs,
        frontmatter_query: None,
    };

    let catalog = markdown_catalog(&docs_root, "Document Catalog", &options)?;
    let links = markdown_link_audit(&docs_root, &options)?;
    let backlinks = markdown_backlinks(&docs_root, &options)?;
    let frontmatter = markdown_frontmatter_inventory(&docs_root, &options)?;
    let headings = markdown_heading_inventory(&docs_root, &options)?;

    let mut markdown = String::new();
    markdown.push_str(&format!("# {}\n\n", title));
    markdown.push_str(&format!("PROOF root: `{}`\n\n", root.display()));
    markdown.push_str(&format!("Docs root: `{}`\n\n", docs_root.display()));
    markdown.push_str("## Health summary\n\n");
    markdown.push_str("| Surface | Count | Issues |\n");
    markdown.push_str("|---------|------:|-------:|\n");
    markdown.push_str(&format!(
        "| Catalog sources | {} | 0 |\n",
        backlinks.source_count
    ));
    markdown.push_str(&format!(
        "| Links | {} | {} |\n",
        links.link_count, links.broken_count
    ));
    markdown.push_str(&format!(
        "| Orphan pages | {} | {} |\n",
        backlinks.source_count, backlinks.orphan_count
    ));
    markdown.push_str(&format!(
        "| Frontmatter pages | {} | 0 |\n",
        frontmatter.page_count
    ));
    markdown.push_str(&format!(
        "| Headings | {} | {} |\n",
        headings.heading_count, headings.duplicate_anchor_count
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
    markdown.push_str(&catalog);
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_link_audit(&links));
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_backlinks(&backlinks));
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_frontmatter_inventory(&frontmatter));
    markdown.push_str("\n---\n\n");
    markdown.push_str(&render_markdown_heading_inventory(&headings));

    if let Some(output) = output {
        fs::write(output, markdown)?;
    } else {
        println!("{markdown}");
    }
    Ok(())
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

    fn unique_temp_file(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{nanos}-{name}"))
    }
}
