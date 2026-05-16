use anyhow::{bail, Result};
use crop_core::{
    markdown_backlinks, markdown_backlinks_for_view_json, markdown_catalog,
    markdown_catalog_for_view_json, markdown_frontmatter_inventory,
    markdown_frontmatter_inventory_for_view_json, markdown_heading_inventory,
    markdown_heading_inventory_for_view_json, markdown_link_audit,
    markdown_link_audit_for_view_json, render_markdown_backlinks,
    render_markdown_frontmatter_inventory, render_markdown_heading_inventory,
    render_markdown_link_audit, CropView, IngestOptions,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::SideInfoFormatArg;

pub fn run(
    root: Option<PathBuf>,
    view: Option<PathBuf>,
    title: String,
    extensions: Vec<String>,
    exclude_dirs: Vec<String>,
    strict: bool,
    format: SideInfoFormatArg,
    output: Option<PathBuf>,
) -> Result<()> {
    if root.is_some() && view.is_some() {
        bail!("--root and --view cannot be combined");
    }

    let docs = if let Some(view) = view {
        status_docs_for_view(view, extensions, exclude_dirs)?
    } else {
        let Some(root) = root else {
            bail!("--root or --view is required");
        };
        status_docs_for_root(root, extensions, exclude_dirs)?
    };

    let rendered = match format {
        SideInfoFormatArg::Markdown => render_status_page(&title, &docs),
        SideInfoFormatArg::Json => serde_json::to_string_pretty(&status_report(&title, &docs))?,
    };
    if let Some(output) = output {
        fs::write(output, rendered)?;
    } else {
        println!("{rendered}");
    }
    if strict {
        let issue_count = docs.issue_count();
        if issue_count > 0 {
            bail!("corpus status found {issue_count} issue(s)");
        }
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

impl StatusDocs {
    fn issue_count(&self) -> usize {
        self.links.broken_count + self.backlinks.orphan_count + self.headings.duplicate_anchor_count
    }
}

#[derive(Debug, Serialize)]
struct CorpusStatusReport {
    schema_version: &'static str,
    title: String,
    input: String,
    issue_count: usize,
    strict_pass: bool,
    surfaces: Vec<CorpusStatusSurface>,
}

#[derive(Debug, Serialize)]
struct CorpusStatusSurface {
    name: &'static str,
    count: usize,
    issues: usize,
}

fn status_report(title: &str, docs: &StatusDocs) -> CorpusStatusReport {
    let surfaces = vec![
        CorpusStatusSurface {
            name: "catalog_sources",
            count: docs.backlinks.source_count,
            issues: 0,
        },
        CorpusStatusSurface {
            name: "links",
            count: docs.links.link_count,
            issues: docs.links.broken_count,
        },
        CorpusStatusSurface {
            name: "orphan_pages",
            count: docs.backlinks.source_count,
            issues: docs.backlinks.orphan_count,
        },
        CorpusStatusSurface {
            name: "frontmatter_pages",
            count: docs.frontmatter.page_count,
            issues: 0,
        },
        CorpusStatusSurface {
            name: "headings",
            count: docs.headings.heading_count,
            issues: docs.headings.duplicate_anchor_count,
        },
    ];
    let issue_count = docs.issue_count();

    CorpusStatusReport {
        schema_version: "crop.corpus-status.v1",
        title: title.to_string(),
        input: docs.label.clone(),
        issue_count,
        strict_pass: issue_count == 0,
        surfaces,
    }
}

fn status_docs_for_root(
    root: PathBuf,
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
        label: root.display().to_string(),
        catalog: markdown_catalog(&root, "Document Catalog", &options)?,
        links: markdown_link_audit(&root, &options)?,
        backlinks: markdown_backlinks(&root, &options)?,
        frontmatter: markdown_frontmatter_inventory(&root, &options)?,
        headings: markdown_heading_inventory(&root, &options)?,
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

fn render_status_page(title: &str, docs: &StatusDocs) -> String {
    let mut markdown = String::new();
    markdown.push_str(&format!("# {}\n\n", title));
    markdown.push_str(&format!("Input: `{}`\n\n", docs.label));
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
    markdown
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn status_renders_combined_markdown_page() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("crop-status.md");

        run(
            Some(fixture.join("src")),
            None,
            "Fixture Status".to_string(),
            vec!["md".to_string()],
            Vec::new(),
            false,
            SideInfoFormatArg::Markdown,
            Some(output.clone()),
        )
        .unwrap();

        let markdown = fs::read_to_string(&output).unwrap();
        assert!(markdown.contains("# Fixture Status"));
        assert!(markdown.contains("## Health summary"));
        assert!(markdown.contains("# Document Catalog"));
        assert!(markdown.contains("# Markdown Link Audit"));
        assert!(markdown.contains("# Markdown Headings"));
        assert!(!markdown.contains("# PROOF Artifact Health"));

        fs::remove_file(output).unwrap();
    }

    #[test]
    fn status_accepts_named_view_input() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("crop-view-status.md");

        run(
            None,
            Some(fixture.join("proof-ready-view.json")),
            "View Status".to_string(),
            Vec::new(),
            Vec::new(),
            false,
            SideInfoFormatArg::Markdown,
            Some(output.clone()),
        )
        .unwrap();

        let markdown = fs::read_to_string(&output).unwrap();
        assert!(markdown.contains("# View Status"));
        assert!(markdown.contains("Input: `"));
        assert!(markdown.contains("# proof-fixture-ready"));
        assert!(markdown.contains("guide.source.md"));
        assert!(!markdown.contains("draft.source.md"));

        fs::remove_file(output).unwrap();
    }

    #[test]
    fn status_rejects_bad_view_input() {
        let result =
            status_docs_for_view(PathBuf::from("missing-view.json"), Vec::new(), Vec::new());

        assert!(result.is_err());
    }

    #[test]
    fn status_strict_fails_after_writing_when_issues_exist() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("crop-status-strict.md");

        let result = run(
            Some(fixture.join("src")),
            None,
            "Strict Status".to_string(),
            vec!["md".to_string()],
            Vec::new(),
            true,
            SideInfoFormatArg::Markdown,
            Some(output.clone()),
        );

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("corpus status found"));
        assert!(fs::read_to_string(&output)
            .unwrap()
            .contains("# Strict Status"));

        fs::remove_file(output).unwrap();
    }

    #[test]
    fn status_strict_allows_clean_corpus() {
        let root = unique_temp_dir("crop-status-clean");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("docs").join("one.md"), "# One\n\n[Two](two.md)\n").unwrap();
        fs::write(root.join("docs").join("two.md"), "# Two\n\n[One](one.md)\n").unwrap();

        run(
            Some(root.join("docs")),
            None,
            "Clean Status".to_string(),
            vec!["md".to_string()],
            Vec::new(),
            true,
            SideInfoFormatArg::Markdown,
            None,
        )
        .unwrap();

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn status_can_emit_json_summary_contract() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("crop-status.json");

        run(
            Some(fixture.join("src")),
            None,
            "Fixture Status".to_string(),
            vec!["md".to_string()],
            Vec::new(),
            false,
            SideInfoFormatArg::Json,
            Some(output.clone()),
        )
        .unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        assert_eq!(json["schema_version"], "crop.corpus-status.v1");
        assert_eq!(json["title"], "Fixture Status");
        assert!(json["issue_count"].as_u64().unwrap() > 0);
        assert_eq!(json["strict_pass"], false);
        assert_eq!(json["surfaces"].as_array().unwrap().len(), 5);

        fs::remove_file(output).unwrap();
    }

    fn unique_temp_file(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{nanos}-{name}"))
    }

    fn unique_temp_dir(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{nanos}-{name}"))
    }
}
