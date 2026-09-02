#[test]
fn open_pitfalls_remain_user_and_consumer_facing() {
    let pitfalls = repo_text(".pitfall/mdcrop-pitfalls.md");
    for id in ["MDCROP-PF-01", "MDCROP-PF-02", "MDCROP-PF-03"] {
        assert_contains(&pitfalls, id);
    }
    for field in [
        "Actor:",
        "Task:",
        "Surface:",
        "Likely mistake:",
        "Consequence:",
        "Owner:",
        "cargo test -p mdcrop-core --test pitfall_policy",
    ] {
        assert_contains(&pitfalls, field);
    }

    assert_contains(&pitfalls, "Context Pack Becomes Answer Authority");
    assert_contains(
        &pitfalls,
        "Fixture Win Becomes General Retrieval Superiority",
    );
    assert_contains(
        &pitfalls,
        "Consumer Adoption Pin Becomes Compatibility Guarantee",
    );

    let boundary_manifest = repo_text("docs/specs/pitfall-boundaries.v1.json");
    for required in [
        "MDCROP-PF-01",
        "MDCROP-PF-02",
        "MDCROP-PF-03",
        "final answer authority",
        "agent may stop checking",
        "broad retrieval superiority",
        "real-corpus benchmark for real-corpus claims",
        "future compatibility guarantee",
        "consumer-specific rehearsal",
    ] {
        assert_contains(&boundary_manifest, required);
    }

    let roles = repo_text(".roles/ROLE.md");
    for required in [
        "Context Boundary Cartographer",
        "Retrieval Benchmark Skeptic",
        "View Contract Keeper",
        "permission for an agent to",
        "stop checking excluded evidence",
    ] {
        assert_contains(&roles, required);
    }

    let product_plan = repo_text("PRODUCT_PLAN.md");
    assert_contains(&product_plan, "evidence graph");
    assert_contains(&product_plan, "context pack for a task");
    assert_contains(&product_plan, "Demonstrates graph-native wins on fixtures");

    let benchmarks = repo_text("docs/BENCHMARKS.md");
    assert_contains(&benchmarks, "fixture");
    assert_contains(&benchmarks, "competitor");
    assert_contains(&benchmarks, "Fixture wins do not prove broad retrieval");

    let consumer_options = repo_text("docs/specs/consumer-integration-options.md");
    assert_contains(&consumer_options, "consumer");
    assert_contains(&consumer_options, "MDCROP");
    assert_contains(&consumer_options, "not a future compatibility");

    let proof_artifacts = repo_text("docs/specs/proof-artifacts.md");
    assert_contains(&proof_artifacts, "PROOF");
    assert_contains(&proof_artifacts, "artifact");

    let views = repo_text("docs/VIEWS.md");
    assert_contains(&views, "examples\\local-views\\");
    assert_contains(&views, "outside `examples\\views\\`");
}

fn repo_text(relative: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    std::fs::read_to_string(root.join(relative)).expect(relative)
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(haystack.contains(needle), "missing {needle}");
}
