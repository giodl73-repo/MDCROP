//! Print the retained synthetic graph and representative native scout responses.
use mdcrop_core::fixture_graph;
use mdcrop_wasm_boundary_probe::fixture_pack;
use serde_json::{json, Value};

fn main() {
    let mut cases = Vec::new();
    for seed in 0..4 {
        for budget in [80, 120, 160, 200] {
            cases.push(json!({"seed":seed,"budget":budget,
                "response":serde_json::from_str::<Value>(&fixture_pack(seed,budget)).unwrap()}));
        }
    }
    for (seed, budget) in [(0, 0), (4, 160), (0, 257), (0, 79)] {
        cases.push(json!({"seed":seed,"budget":budget,
            "response":serde_json::from_str::<Value>(&fixture_pack(seed,budget)).unwrap()}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({
        "schema":"mdcrop.wasm-boundary-native-fixture.v1",
        "scope":"synthetic fixture only; native observations, not browser runtime or consumer authority",
        "graph":fixture_graph(),"cases":cases})).unwrap());
}
