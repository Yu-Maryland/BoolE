use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::*;

pub struct BottomUpExtractor;
impl Extractor for BottomUpExtractor {
    fn extract(&self, egraph: &EGraph, _roots: &[ClassId]) -> ExtractionResult {
        let mut result = ExtractionResult::default();
        let mut costs = FxHashMap::<ClassId, Cost>::with_capacity_and_hasher(
            egraph.classes().len(),
            Default::default(),
        );
        let mut did_something = false;

        loop {
            let mut classes_info: Vec<_> = egraph.classes().values().collect();
            let mut rng = thread_rng();
            classes_info.shuffle(&mut rng);

            for class in classes_info {
            for node in &class.nodes {
                print!("{} ", node);
                let cost = result.node_sum_cost_n(egraph, &egraph[node], &costs);
                if &cost > costs.get(&class.id).unwrap_or(&Neg_INFINITY) {
                result.choose(class.id.clone(), node.clone());
                costs.insert(class.id.clone(), cost);
                did_something = true;
                }
            }
            }

            if did_something {
            did_something = false;
            } else {
            break;
            }
        }

        result
    }
}
