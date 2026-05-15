use rctx_core::RCTX_VERSION;
use rgraph_core::{
    shortest_connector_path, undirected_boundary_metrics, undirected_cluster_summaries,
};
use ropt_core::{exact_budgeted_selection, BudgetItem, RoptError, SeedPart};
use rstat_core::scoring::{weighted_component_score, ScoringError};
use rstat_core::summary::{summary_stats, SummaryError};
use serde::{Deserialize, Serialize};
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
    pub relevance: f64,
    pub token_estimate: usize,
    pub reason: CropReason,
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

pub fn shared_kernel_probe(seed: usize) -> (&'static str, SeedPart) {
    (RCTX_VERSION, SeedPart::Usize(seed))
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

    fn unit(id: &str, relevance: f64, token_estimate: usize) -> EvidenceUnit {
        EvidenceUnit {
            id: id.to_string(),
            source_id: format!("source:{id}"),
            relevance,
            token_estimate,
        }
    }
}
