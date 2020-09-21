pub mod traits;

use crate::traits::candidate::Candidate;
use crate::traits::error::Error;
use crate::traits::network::{Network, Node};
use crate::traits::query::{Query, QueryBuilder, QueryContext, QueryResponse};
use crate::traits::sampler::Sampler;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

/// Snowball loop implements snow ball algorithm and identifies
/// a candidate preferred by majority of nodes in the network.
pub fn snow_ball_loop<L, N, NW, S, C, QC, QB, QR, Q, F, E>(
    location: L,
    mut network: NW,
    mut sampler: S,
    current_candidate: C,
    candidates: HashSet<C>,
    query_context: QC,
    mut query_builder: QB,
    acceptance_threshold: usize,
    sample_size: u64,
    query_threshold: usize,
    query_filter: fn(query: Q, originating_node: &N) -> bool,
) -> Result<C, E>
where
    L: Serialize + DeserializeOwned + PartialEq,
    C: Candidate,
    QC: QueryContext,
    QR: QueryResponse<Candidate = C>,
    Q: Query<Location = L, Context = QC, Candidate = C>,
    N: Node<Query = Q, QueryResponse = QR, Error = E>,
    NW: Network<Node = N>,
    S: Sampler<SamplingType = u64, Error = E>,
    QB: QueryBuilder<Context = QC, Location = L, Candidate = C, Query = Q>,
    E: Error,
{
    let mut decided = false;

    let mut current_preferred_candidate = current_candidate;
    let mut last_chosen_candidate = current_candidate;
    let mut candidate_preference: HashMap<C, usize> = HashMap::new();
    let mut acceptance_count: usize = 0;

    if candidates.is_empty() {
        return Ok(current_preferred_candidate);
    }

    network.update_preferred_candidate(current_preferred_candidate);
    network.register_query_filter(query_filter)?;

    while !decided {
        let nodes = network.node_ids();
        let sampled_nodes = sampler.sample(nodes, sample_size)?;
        let query = query_builder.build_query(&current_candidate, &location, &query_context);
        let results = network.execute_query(sampled_nodes, query).unwrap();
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
                    network.update_preferred_candidate(current_preferred_candidate);
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

    Ok(current_preferred_candidate)
}
