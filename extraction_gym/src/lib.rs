use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::HashMap;
use egraph_serialize::*;
use ordered_float::NotNan;
pub const INFINITY: Cost = 32767;
pub const Neg_INFINITY: Cost = -32767;
pub mod bottom_up;
pub mod faster_bottom_up;
pub mod faster_greedy_dag;
pub mod faster_greedy_dag_fa;
pub mod faster_greedy_dag_fa_mt;
pub mod global_greedy_dag;
pub mod greedy_dag;

// Allowance for floating point values to be considered equal
pub const EPSILON_ALLOWANCE: f64 = 0.00001;

pub trait Extractor: Sync {
    fn extract(&self, egraph: &EGraph, roots: &[ClassId]) -> ExtractionResult;

    fn boxed(self) -> Box<dyn Extractor>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }
}

pub trait MapGet<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
}

impl<K, V> MapGet<K, V> for HashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        HashMap::get(self, key)
    }
}

impl<K, V> MapGet<K, V> for FxHashMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        FxHashMap::get(self, key)
    }
}

impl<K, V> MapGet<K, V> for IndexMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn get(&self, key: &K) -> Option<&V> {
        IndexMap::get(self, key)
    }
}

#[derive(Default, Clone)]
pub struct ExtractionResult {
    pub choices: IndexMap<ClassId, NodeId>,
}

#[derive(Clone, Copy)]
enum Status {
    Doing,
    Done,
}

impl ExtractionResult {
    pub fn check(&self, egraph: &EGraph) {
        // println!("choices1: {:?}",self.choices);
        // should be a root
        assert!(!egraph.root_eclasses.is_empty());

        // All roots should be selected.
        for cid in egraph.root_eclasses.iter() {
            assert!(self.choices.contains_key(cid));
        }

        let mut fst_op: FxHashSet<NodeId> = FxHashSet::default();
        let mut snd_op: FxHashSet<NodeId> = FxHashSet::default();
        for (node_id, node) in &egraph.nodes {
            if node.op == "fst" {
                fst_op.insert(node_id.clone());
            } else if node.op == "snd" {
                snd_op.insert(node_id.clone());
            }
        }

        let mut pair: FxHashSet<(NodeId, NodeId)> = FxHashSet::default(); // (fa, xor, maj)
        'outer:for fst in &fst_op {
            for snd in &snd_op {
                if egraph.nodes[fst].children == egraph.nodes[snd].children {
                    let cid1 = egraph.nid_to_cid(fst);
                    let cid2 = egraph.nid_to_cid(snd);
                    let select1 = &self.choices[cid1]==fst;
                    let select2 = &self.choices[cid2]==snd;
                    // make sure if fst is selected, snd is selected or if fst is not selected, snd is not selected
                    if (select1 != select2) {
                        eprintln!("\x1b[31mWarning: fst and snd should be selected together or not selected together: {:?}-{:?}, {:?}-{:?}\x1b[0m", fst, self.choices[cid1], snd, self.choices[cid2]);
                        break 'outer;
                    }
                }
            }
        }

        // for fst in &fst_op {
        //     for snd in &snd_op {
        //         if egraph.nodes[fst].children == egraph.nodes[snd].children {
        //             fst_nmap.insert(fst.clone(), snd.clone());
        //             snd_nmap.insert(snd.clone(), fst.clone());
        //         }
        //     }
        // }

        // No cycles
        assert!(self.find_cycles(&egraph, &egraph.root_eclasses).is_empty());

        // Nodes should match the class they are selected into.
        for (cid, nid) in &self.choices {
            let node = &egraph[nid];
            assert!(node.eclass == *cid);
        }

        // All the nodes the roots depend upon should be selected.
        let mut todo: Vec<ClassId> = egraph.root_eclasses.to_vec();
        let mut visited: FxHashSet<ClassId> = Default::default();
        while let Some(cid) = todo.pop() {
            if !visited.insert(cid.clone()) {
                continue;
            }
            assert!(self.choices.contains_key(&cid));

            for child in &egraph[&self.choices[&cid]].children {
                todo.push(egraph.nid_to_cid(child).clone());
            }
        }
    }

    pub fn choose(&mut self, class_id: ClassId, node_id: NodeId) {
        self.choices.insert(class_id, node_id);
    }

    pub fn find_cycles(&self, egraph: &EGraph, roots: &[ClassId]) -> Vec<ClassId> {
        // let mut status = vec![Status::Todo; egraph.classes().len()];
        let mut status = IndexMap::<ClassId, Status>::default();
        let mut cycles = vec![];
        for root in roots {
            // let root_index = egraph.classes().get_index_of(root).unwrap();
            self.cycle_dfs(egraph, root, &mut status, &mut cycles)
        }
        cycles
    }

    fn cycle_dfs(
        &self,
        egraph: &EGraph,
        class_id: &ClassId,
        status: &mut IndexMap<ClassId, Status>,
        cycles: &mut Vec<ClassId>,
    ) {
        match status.get(class_id).cloned() {
            Some(Status::Done) => (),
            Some(Status::Doing) => cycles.push(class_id.clone()),
            None => {
                status.insert(class_id.clone(), Status::Doing);
                let node_id = &self.choices[class_id];
                let node = &egraph[node_id];
                for child in &node.children {
                    let child_cid = egraph.nid_to_cid(child);
                    self.cycle_dfs(egraph, child_cid, status, cycles)
                }
                status.insert(class_id.clone(), Status::Done);
            }
        }
    }

    pub fn tree_cost(&self, egraph: &EGraph, roots: &[ClassId]) -> Cost {
        let node_roots = roots
            .iter()
            .map(|cid| self.choices[cid].clone())
            .collect::<Vec<NodeId>>();
        self.tree_cost_rec(egraph, &node_roots, &mut HashMap::new())
    }

    fn tree_cost_rec(
        &self,
        egraph: &EGraph,
        roots: &[NodeId],
        memo: &mut HashMap<NodeId, Cost>,
    ) -> Cost {
        let mut cost = Cost::default();
        for root in roots {
            if let Some(c) = memo.get(root) {
                cost += *c;
                continue;
            }
            let class = egraph.nid_to_cid(root);
            let node = &egraph[&self.choices[class]];
            let inner = node.cost + self.tree_cost_rec(egraph, &node.children, memo);
            memo.insert(root.clone(), inner);
            cost += inner;
        }
        cost
    }

    // this will loop if there are cycles
    pub fn dag_cost(&self, egraph: &EGraph, roots: &[ClassId]) -> Cost {
        let mut costs: IndexMap<ClassId, Cost> = IndexMap::new();
        let mut todo: Vec<ClassId> = roots.to_vec();
        while let Some(cid) = todo.pop() {
            let node_id = &self.choices[&cid];
            let node = &egraph[node_id];
            if costs.insert(cid.clone(), node.cost).is_some() {
                continue;
            }
            for child in &node.children {
                todo.push(egraph.nid_to_cid(child).clone());
            }
        }
        costs.values().sum()
    }

    pub fn node_sum_cost<M>(&self, egraph: &EGraph, node: &Node, costs: &M) -> Cost
    where
        M: MapGet<ClassId, Cost>,
    {
        node.cost
            + node
                .children
                .iter()
                .map(|n| {
                    let cid = egraph.nid_to_cid(n);
                    costs.get(cid).unwrap_or(&INFINITY)
                })
                .sum::<Cost>()
    }

    pub fn node_sum_cost_n<M>(&self, egraph: &EGraph, node: &Node, costs: &M) -> Cost
    where
        M: MapGet<ClassId, Cost>,
    {
        node.cost
            + node
                .children
                .iter()
                .map(|n| {
                    let cid = egraph.nid_to_cid(n);
                    costs.get(cid).unwrap_or(&Neg_INFINITY)
                })
                .sum::<Cost>()
    }
}
