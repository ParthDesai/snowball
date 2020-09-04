pub mod traits;

use crate::traits::candidate::Candidate;
use crate::traits::network::{Network, NetworkQueryExecutor, Node};
use crate::traits::query::{Query, QueryBuilder, QueryContext, QueryResponse};
use crate::traits::sampler::Sampler;
use crossbeam_channel::Sender;
use parking_lot::Mutex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub fn snow_ball_loop<L, N, NW, NQE, S, C, QC, QB, QR, Q>(
    location: L,
    network: NW,
    mut network_query_executor: NQE,
    mut sampler: S,
    current_candidate: C,
    candidates: HashSet<C>,
    query_context: QC,
    mut query_builder: QB,
    acceptance_threshold: usize,
    sample_size: u64,
    query_threshold: usize,
) -> C
where
    L: Serialize + DeserializeOwned + PartialEq,
    C: Candidate,
    QC: QueryContext,
    QR: QueryResponse<Candidate = C>,
    Q: Query<Location = L, Context = QC, Candidate = C>,
    N: Node<Query = Q, QueryResponse = QR>,
    NW: Network<Node = N>,
    NQE: NetworkQueryExecutor<Node = N>,
    S: Sampler<SamplingType = N>,
    QB: QueryBuilder<Context = QC, Location = L, Candidate = C, Query = Q>,
{
    let mut decided = false;

    let mut current_preferred_candidate = current_candidate;
    let mut last_chosen_candidate = current_candidate;
    let mut candidate_preference: HashMap<C, usize> = HashMap::new();
    let mut acceptance_count: usize = 0;

    if candidates.is_empty() {
        return current_preferred_candidate;
    }

    while !decided {
        let nodes = network.nodes();
        let sampled_nodes = sampler.sample(nodes, sample_size).unwrap();
        let query = query_builder.build_query(&current_candidate, &location, &query_context);
        let results = network_query_executor
            .execute_query(sampled_nodes, query)
            .unwrap();
        let mut frequency: HashMap<C, usize> = HashMap::new();

        // Count how many QueryResponse contains a particular candidate
        for result in results {
            if frequency.contains_key(result.preferred_candidate()) {
                frequency.insert(
                    *result.preferred_candidate(),
                    frequency[result.preferred_candidate()] + 1,
                );
            } else {
                frequency.insert(*result.preferred_candidate(), 1);
            }
        }

        // If candidate is returned by other nodes for more than query_threshold,
        // we increment its preference count
        for (candidate, f) in frequency {
            if f > query_threshold {
                if candidate_preference.contains_key(&candidate) {
                    candidate_preference.insert(candidate, candidate_preference[&candidate] + 1);
                } else {
                    candidate_preference.insert(candidate, 1);
                }

                if *candidate_preference
                    .get(&current_preferred_candidate)
                    .unwrap_or(&0)
                    < candidate_preference[&candidate]
                {
                    current_preferred_candidate = candidate;
                }

                if candidate == last_chosen_candidate {
                    acceptance_count = acceptance_count + 1;
                } else {
                    acceptance_count = 0;
                }
                last_chosen_candidate = candidate;

                break;
            }
        }

        if acceptance_count > acceptance_threshold {
            decided = true;
        }
    }

    current_preferred_candidate
}
