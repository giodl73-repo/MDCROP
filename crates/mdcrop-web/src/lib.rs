//! Fixture-only browser policy. Native selection and consumer contracts stay unchanged.
use mdcrop_core::{fixture_graph, mdcrop_context, MdcropOptions};
use serde::Deserialize;
use serde_json::{json, Value};

pub const MAX_REQUEST_BYTES: usize = 1024;
pub const SCHEMA: &str = "mdcrop.fixture-explorer.v1";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    seed: String,
    budget: String,
}

fn failure(kind: &str, field: Option<&str>, message: &str) -> Value {
    json!({"schema":SCHEMA,"ok":false,"diagnostic":{"kind":kind,"field":field,"message":message}})
}
fn decimal(text: &str, field: &str, min: u32, max: u32) -> Result<u32, Value> {
    if text.is_empty() || !text.bytes().all(|c| c.is_ascii_digit()) {
        return Err(failure(
            "controls",
            Some(field),
            &format!("{field} must be a decimal whole number from {min} to {max}."),
        ));
    }
    text.parse::<u32>()
        .ok()
        .filter(|n| *n >= min && *n <= max)
        .ok_or_else(|| {
            failure(
                "controls",
                Some(field),
                &format!("{field} must be from {min} to {max}."),
            )
        })
}
fn report(input: &str) -> Value {
    if input.len() > MAX_REQUEST_BYTES {
        return failure("limit", None, "Request exceeds 1,024 UTF-8 bytes.");
    }
    let request: Request = match serde_json::from_str(input) {
        Ok(request) => request,
        Err(error) => {
            return json!({"schema":SCHEMA,"ok":false,"diagnostic":{"kind":"request","message":error.to_string(),"line":error.line(),"column":error.column()}})
        }
    };
    let seed = match decimal(&request.seed, "seed", 0, 3) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let budget = match decimal(&request.budget, "budget", 1, 256) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let graph = fixture_graph();
    let pack = match mdcrop_context(
        &graph,
        MdcropOptions {
            seed: seed as usize,
            token_budget: budget as usize,
        },
    ) {
        Ok(pack) => pack,
        Err(error) => return failure("kernel", None, &error.to_string()),
    };
    let selected: Vec<bool> = graph
        .units
        .iter()
        .map(|u| pack.units.iter().any(|p| p.id == u.id))
        .collect();
    let units: Vec<Value> = graph
        .units
        .iter()
        .enumerate()
        .map(|(i, unit)| {
            json!({
                "index":i,"unit":unit,"selected":selected[i],
                "reason":pack.units.iter().find(|p|p.id==unit.id).map(|p|p.reason),
            })
        })
        .collect();
    let mut edges = Vec::new();
    for (from, neighbors) in graph.adjacency.iter().enumerate() {
        for &to in neighbors {
            if from < to {
                edges.push(json!({"from":from,"to":to,"state":if selected[from] && selected[to]{"inside"}else if selected[from] || selected[to]{"boundary"}else{"outside"}}));
            }
        }
    }
    json!({"schema":SCHEMA,"ok":true,"controls":{"seed":seed,"budget":budget},
        "fixture":"synthetic-bridge-and-noise.v1","units":units,"edges":edges,
        "selected_count":pack.units.len(),"excluded_count":graph.units.len()-pack.units.len(),
        "over_budget":pack.token_estimate>budget as usize,
        "budget_overrun":pack.token_estimate.saturating_sub(budget as usize),"pack":pack})
}

#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn explore(input: &str) -> String {
    report(input).to_string()
}

pub fn fixture_json() -> String {
    serde_json::to_string(&fixture_graph()).expect("fixed fixture serializes")
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(seed: &str, budget: &str) -> Value {
        report(&json!({"seed":seed,"budget":budget}).to_string())
    }
    #[test]
    fn all_controls_preserve_native_pack_partition_and_edge_boundary() {
        for seed in 0..4 {
            for budget in 1..=256 {
                let r = run(&seed.to_string(), &budget.to_string());
                assert_eq!(r["ok"], true);
                let native = mdcrop_context(
                    &fixture_graph(),
                    MdcropOptions {
                        seed,
                        token_budget: budget,
                    },
                )
                .unwrap();
                assert_eq!(r["pack"], serde_json::to_value(native).unwrap());
                assert_eq!(
                    r["selected_count"].as_u64().unwrap() + r["excluded_count"].as_u64().unwrap(),
                    4
                );
                assert_eq!(
                    r["edges"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter(|e| e["state"] == "boundary")
                        .count() as u64,
                    r["pack"]["boundary_cut_edges"].as_u64().unwrap()
                );
            }
        }
    }
    #[test]
    fn bridge_selection_and_seed_overrun_are_not_rewritten() {
        let r = run("0", "160");
        assert_eq!(r["selected_count"], 3);
        assert_eq!(r["units"][1]["reason"], "BridgeConnector");
        assert_eq!(r["units"][3]["selected"], false);
        let r = run("0", "79");
        assert_eq!(r["over_budget"], true);
        assert_eq!(r["budget_overrun"], 1);
        assert_eq!(r["pack"]["token_estimate"], 80);
    }
    #[test]
    fn decimals_reject_coercion_and_wrong_ranges() {
        for text in [
            "",
            "-1",
            "+1",
            "1.5",
            "NaN",
            "Infinity",
            "0xA0",
            "1e2",
            " 80 ",
            "999999999999999999999999",
        ] {
            assert_eq!(run("0", text)["ok"], false, "{text}");
        }
        for (seed, budget) in [("4", "160"), ("0", "0"), ("0", "257")] {
            assert_eq!(run(seed, budget)["ok"], false);
        }
    }
    #[test]
    fn malformed_missing_unknown_duplicate_and_wrong_types_fail() {
        for input in [
            "[broken",
            "{}",
            r#"{"seed":"0","budget":160}"#,
            r#"{"seed":false,"budget":"160"}"#,
            r#"{"seed":"0","budget":null}"#,
            r#"{"seed":"0","budget":"160","graph":{}}"#,
            r#"{"seed":"0","seed":"1","budget":"160"}"#,
        ] {
            assert_eq!(report(input)["ok"], false);
        }
        assert_eq!(report(&"é".repeat(513))["diagnostic"]["kind"], "limit");
    }
    #[test]
    fn provenance_text_hashes_and_sources_are_unchanged_for_excluded_units() {
        let r = run("0", "80");
        let graph = fixture_graph();
        for (i, u) in graph.units.iter().enumerate() {
            assert_eq!(r["units"][i]["unit"], serde_json::to_value(u).unwrap());
        }
        assert_eq!(r["units"][2]["selected"], false);
    }
}
