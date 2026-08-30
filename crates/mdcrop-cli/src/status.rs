use anyhow::{bail, Result};
use mdcrop_core::{
    markdown_backlinks, markdown_backlinks_for_view_json, markdown_catalog,
    markdown_catalog_for_view_json, markdown_frontmatter_inventory,
    markdown_frontmatter_inventory_for_view_json, markdown_heading_inventory,
    markdown_heading_inventory_for_view_json, markdown_link_audit,
    markdown_link_audit_for_view_json, render_markdown_backlinks,
    render_markdown_frontmatter_inventory, render_markdown_heading_inventory,
    render_markdown_link_audit, IngestOptions, MdcropView,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

use crate::{SideInfoFormatArg, StatusStrictPolicyArg};

pub struct RunOptions {
    pub root: Option<PathBuf>,
    pub view: Option<PathBuf>,
    pub title: String,
    pub extensions: Vec<String>,
    pub exclude_dirs: Vec<String>,
    pub strict: bool,
    pub strict_policies: Vec<StatusStrictPolicyArg>,
    pub format: SideInfoFormatArg,
    pub output: Option<PathBuf>,
}

pub fn run(options: RunOptions) -> Result<()> {
    let RunOptions {
        root,
        view,
        title,
        extensions,
        exclude_dirs,
        strict,
        strict_policies,
        format,
        output,
    } = options;

    if root.is_some() && view.is_some() {
        bail!("--root and --view cannot be combined");
    }
    if !strict && !strict_policies.is_empty() {
        bail!("--strict-on requires --strict");
    }

    let docs = if let Some(view) = view {
        status_docs_for_view(view, extensions, exclude_dirs)?
    } else {
        let Some(root) = root else {
            bail!("--root or --view is required");
        };
        status_docs_for_root(root, extensions, exclude_dirs)?
    };

    let strict_policy = strict_policy(strict_policies);
    let rendered = match format {
        SideInfoFormatArg::Markdown => render_status_page(&title, &docs),
        SideInfoFormatArg::Json => {
            serde_json::to_string_pretty(&status_report(&title, &docs, &strict_policy))?
        }
    };
    if let Some(output) = output {
        fs::write(output, rendered)?;
    } else {
        println!("{rendered}");
    }
    if strict {
        let issue_count = docs.strict_issue_count(&strict_policy);
        if issue_count > 0 {
            bail!("corpus status found {issue_count} issue(s)");
        }
    }
    Ok(())
}

struct StatusDocs {
    label: String,
    catalog: String,
    links: mdcrop_core::MarkdownLinkAudit,
    backlinks: mdcrop_core::MarkdownBacklinkReport,
    frontmatter: mdcrop_core::MarkdownFrontmatterInventory,
    headings: mdcrop_core::MarkdownHeadingInventory,
}

impl StatusDocs {
    fn issue_count(&self) -> usize {
        self.links.broken_count + self.backlinks.orphan_count + self.headings.duplicate_anchor_count
    }

    fn strict_issue_count(&self, policies: &[StatusStrictPolicy]) -> usize {
        policies.iter().map(|policy| policy.issue_count(self)).sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusStrictPolicy {
    BrokenLinks,
    OrphanPages,
    DuplicateAnchors,
}

impl StatusStrictPolicy {
    fn name(self) -> &'static str {
        match self {
            Self::BrokenLinks => "broken_links",
            Self::OrphanPages => "orphan_pages",
            Self::DuplicateAnchors => "duplicate_anchors",
        }
    }

    fn issue_count(self, docs: &StatusDocs) -> usize {
        match self {
            Self::BrokenLinks => docs.links.broken_count,
            Self::OrphanPages => docs.backlinks.orphan_count,
            Self::DuplicateAnchors => docs.headings.duplicate_anchor_count,
        }
    }
}

fn strict_policy(policies: Vec<StatusStrictPolicyArg>) -> Vec<StatusStrictPolicy> {
    if policies.is_empty() {
        return vec![
            StatusStrictPolicy::BrokenLinks,
            StatusStrictPolicy::OrphanPages,
            StatusStrictPolicy::DuplicateAnchors,
        ];
    }

    let mut selected = Vec::new();
    for policy in policies.into_iter().map(|policy| match policy {
        StatusStrictPolicyArg::BrokenLinks => StatusStrictPolicy::BrokenLinks,
        StatusStrictPolicyArg::OrphanPages => StatusStrictPolicy::OrphanPages,
        StatusStrictPolicyArg::DuplicateAnchors => StatusStrictPolicy::DuplicateAnchors,
    }) {
        if !selected.contains(&policy) {
            selected.push(policy);
        }
    }
    selected
}

#[derive(Debug, Serialize)]
struct CorpusStatusReport {
    schema_version: &'static str,
    title: String,
    input: String,
    issue_count: usize,
    strict_issue_count: usize,
    strict_pass: bool,
    strict_policy: Vec<&'static str>,
    surfaces: Vec<CorpusStatusSurface>,
}

#[derive(Debug, Serialize)]
struct CorpusStatusSurface {
    name: &'static str,
    count: usize,
    issues: usize,
}

fn status_report(
    title: &str,
    docs: &StatusDocs,
    strict_policy: &[StatusStrictPolicy],
) -> CorpusStatusReport {
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
    let strict_issue_count = docs.strict_issue_count(strict_policy);

    CorpusStatusReport {
        schema_version: "mdcrop.corpus-status.v1",
        title: title.to_string(),
        input: docs.label.clone(),
        issue_count,
        strict_issue_count,
        strict_pass: strict_issue_count == 0,
        strict_policy: strict_policy.iter().map(|policy| policy.name()).collect(),
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
    let mut recipe: MdcropView = serde_json::from_str(&input)?;
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
        let output = unique_temp_file("mdcrop-status.md");

        run(RunOptions {
            root: Some(fixture.join("src")),
            view: None,
            title: "Fixture Status".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: false,
            strict_policies: Vec::new(),
            format: SideInfoFormatArg::Markdown,
            output: Some(output.clone()),
        })
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
        let output = unique_temp_file("mdcrop-view-status.md");

        run(RunOptions {
            root: None,
            view: Some(fixture.join("proof-ready-view.json")),
            title: "View Status".to_string(),
            extensions: Vec::new(),
            exclude_dirs: Vec::new(),
            strict: false,
            strict_policies: Vec::new(),
            format: SideInfoFormatArg::Markdown,
            output: Some(output.clone()),
        })
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
        let output = unique_temp_file("mdcrop-status-strict.md");

        let result = run(RunOptions {
            root: Some(fixture.join("src")),
            view: None,
            title: "Strict Status".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: true,
            strict_policies: Vec::new(),
            format: SideInfoFormatArg::Markdown,
            output: Some(output.clone()),
        });

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
        let root = unique_temp_dir("mdcrop-status-clean");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("docs").join("one.md"), "# One\n\n[Two](two.md)\n").unwrap();
        fs::write(root.join("docs").join("two.md"), "# Two\n\n[One](one.md)\n").unwrap();

        run(RunOptions {
            root: Some(root.join("docs")),
            view: None,
            title: "Clean Status".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: true,
            strict_policies: Vec::new(),
            format: SideInfoFormatArg::Markdown,
            output: None,
        })
        .unwrap();

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn status_can_emit_json_summary_contract() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let output = unique_temp_file("mdcrop-status.json");

        run(RunOptions {
            root: Some(fixture.join("src")),
            view: None,
            title: "Fixture Status".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: false,
            strict_policies: Vec::new(),
            format: SideInfoFormatArg::Json,
            output: Some(output.clone()),
        })
        .unwrap();

        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&output).unwrap()).unwrap();
        assert_eq!(json["schema_version"], "mdcrop.corpus-status.v1");
        assert_eq!(json["title"], "Fixture Status");
        assert!(json["issue_count"].as_u64().unwrap() > 0);
        assert_eq!(json["strict_issue_count"], json["issue_count"]);
        assert_eq!(json["strict_pass"], false);
        assert_eq!(json["strict_policy"].as_array().unwrap().len(), 3);
        assert_eq!(json["surfaces"].as_array().unwrap().len(), 5);

        fs::remove_file(output).unwrap();
    }

    #[test]
    fn status_root_json_matches_golden_contract() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let mut docs =
            status_docs_for_root(fixture.join("src"), vec!["md".to_string()], Vec::new()).unwrap();
        docs.label = "examples\\proof-fixture\\src".to_string();

        assert_status_golden(
            status_report("Corpus Status", &docs, &strict_policy(vec![])),
            include_str!("../../../examples/proof-fixture/golden/status-root.json"),
        );
    }

    #[test]
    fn status_view_json_matches_golden_contract() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..");
        let fixture = repo.join("examples").join("proof-fixture");
        let mut docs = status_docs_for_view(
            fixture.join("proof-ready-view.json"),
            Vec::new(),
            Vec::new(),
        )
        .unwrap();
        docs.label = "examples\\proof-fixture\\proof-ready-view.json".to_string();

        assert_status_golden(
            status_report(
                "Ready View Status",
                &docs,
                &strict_policy(vec![StatusStrictPolicyArg::BrokenLinks]),
            ),
            include_str!("../../../examples/proof-fixture/golden/status-view.json"),
        );
    }

    #[test]
    fn status_strict_can_gate_selected_issue_classes() {
        let root = unique_temp_dir("mdcrop-status-policy");
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(root.join("docs").join("one.md"), "# One\n\n[Two](two.md)\n").unwrap();
        fs::write(root.join("docs").join("two.md"), "# Two\n").unwrap();
        fs::write(root.join("docs").join("orphan.md"), "# Orphan\n").unwrap();

        run(RunOptions {
            root: Some(root.join("docs")),
            view: None,
            title: "Selected Policy".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: true,
            strict_policies: vec![StatusStrictPolicyArg::BrokenLinks],
            format: SideInfoFormatArg::Markdown,
            output: None,
        })
        .unwrap();

        let result = run(RunOptions {
            root: Some(root.join("docs")),
            view: None,
            title: "Selected Policy".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: true,
            strict_policies: vec![StatusStrictPolicyArg::OrphanPages],
            format: SideInfoFormatArg::Markdown,
            output: None,
        });

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("corpus status found"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn status_strict_policy_requires_strict_mode() {
        let result = run(RunOptions {
            root: Some(PathBuf::from("unused")),
            view: None,
            title: "Invalid Policy".to_string(),
            extensions: vec!["md".to_string()],
            exclude_dirs: Vec::new(),
            strict: false,
            strict_policies: vec![StatusStrictPolicyArg::BrokenLinks],
            format: SideInfoFormatArg::Markdown,
            output: None,
        });

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("--strict-on requires --strict"));
    }

    #[test]
    fn status_strict_policy_deduplicates_repeated_selectors() {
        let policy = strict_policy(vec![
            StatusStrictPolicyArg::BrokenLinks,
            StatusStrictPolicyArg::BrokenLinks,
            StatusStrictPolicyArg::OrphanPages,
        ]);

        assert_eq!(policy.len(), 2);
        assert!(matches!(policy[0], StatusStrictPolicy::BrokenLinks));
        assert!(matches!(policy[1], StatusStrictPolicy::OrphanPages));
    }

    fn assert_status_golden(report: CorpusStatusReport, golden: &str) {
        let actual = serde_json::to_value(report).unwrap();
        let expected: serde_json::Value = serde_json::from_str(golden).unwrap();
        assert_eq!(actual, expected);
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
