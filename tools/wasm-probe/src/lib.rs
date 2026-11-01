//! Compile/link scout over the unchanged fixture/kernel; not a released UI API.
use mdcrop_core::{fixture_graph, mdcrop_context, MdcropOptions};
use serde_json::json;

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn fixture_pack(seed: u32, budget: u32) -> String {
    if budget > 256 {
        return json!({"ok":false,"error":"probe budget limit is 256"}).to_string();
    }
    match mdcrop_context(
        &fixture_graph(),
        MdcropOptions {
            seed: seed as usize,
            token_budget: budget as usize,
        },
    ) {
        Ok(pack) => {
            json!({"ok":true,"over_budget":pack.token_estimate > budget as usize,"pack":pack})
                .to_string()
        }
        Err(error) => json!({"ok":false,"error":error.to_string()}).to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pack(seed: u32, budget: u32) -> serde_json::Value {
        serde_json::from_str(&fixture_pack(seed, budget)).unwrap()
    }
    #[test]
    fn bridge_fixture_retains_provenance_and_excludes_noise() {
        let result = pack(0, 160);
        assert_eq!(result["ok"], true);
        let units = result["pack"]["units"].as_array().unwrap();
        assert_eq!(
            units
                .iter()
                .map(|u| u["id"].as_str().unwrap())
                .collect::<Vec<_>>(),
            vec!["task-seed", "bridge-note", "target-note"]
        );
        assert_eq!(units[1]["reason"], "BridgeConnector");
        for unit in units {
            let graph = fixture_graph();
            let source = graph
                .units
                .iter()
                .find(|source| source.id == unit["id"])
                .unwrap();
            assert_eq!(unit["content_hash"], source.content_hash);
            assert_eq!(unit["source_id"], source.source_id);
            assert_eq!(unit["text"], source.text);
        }
        assert_eq!(result["pack"]["token_estimate"], 160);
    }
    #[test]
    fn seed_overrun_is_exposed_without_rewriting_native_semantics() {
        let result = pack(0, 79);
        assert_eq!(result["ok"], true);
        assert_eq!(result["over_budget"], true);
        assert_eq!(result["pack"]["token_estimate"], 80);
    }
    #[test]
    fn invalid_controls_and_every_admitted_seed_budget_are_bounded() {
        assert_eq!(pack(4, 160)["ok"], false);
        assert_eq!(pack(0, 0)["ok"], false);
        assert_eq!(pack(0, 257)["ok"], false);
        for seed in 0..4 {
            for budget in 1..=256 {
                let result = pack(seed, budget);
                assert_eq!(result["ok"], true, "seed={seed}, budget={budget}");
                assert!(result["pack"]["units"].as_array().unwrap().len() <= 4);
            }
        }
    }

    #[test]
    fn retained_fixture_and_twenty_native_observations_do_not_drift() {
        let retained: serde_json::Value = serde_json::from_str(include_str!(
            "../../../context/waves/2026-10-31-wasm-boundary/native-fixture.json"
        ))
        .unwrap();
        assert_eq!(
            retained["graph"],
            serde_json::to_value(fixture_graph()).unwrap()
        );
        for case in retained["cases"].as_array().unwrap() {
            assert_eq!(
                pack(
                    case["seed"].as_u64().unwrap() as u32,
                    case["budget"].as_u64().unwrap() as u32
                ),
                case["response"]
            );
        }
    }
}
