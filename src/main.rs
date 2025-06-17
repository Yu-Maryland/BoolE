// cargo run --features "faster_greedy_dag_fa_mt"  --release -- mul4 True
// cargo run --features "faster_greedy_dag_fa_mt"  --release -- mul4_map True

mod lib;
mod extractor;
mod static_rules;
mod rule1;
// mod lp;
use std::path::PathBuf;
use std::fs;
use std::env;
use std::fs::{File};
use std::io::{BufWriter};
use std::io::BufReader;
use std::fs::OpenOptions;
use std::io::{self, Write,BufRead};
use std::fmt::Display;
use egraph_serialize::ClassId;
use egraph_serialize::EGraph as SerializedEGraph;
use egraph_serialize::NodeId;
// use hashbrown::HashSet;
use indexmap::IndexMap;
use ordered_float::NotNan;
use egraph_mapping::{SimpleLanguage, convert_to_simple_language_enum, get_node_type};
use rayon::result;
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;
use std::path::Path;
use std::process::Command;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use anyhow::Context;
use egg::*;
use indexmap::IndexSet;

// Custom struct to hash the children arrays
struct ChildrenHash<'a>(&'a serde_json::Value);

impl<'a> PartialEq for ChildrenHash<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> Eq for ChildrenHash<'a> {}

impl<'a> Hash for ChildrenHash<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        serde_json::to_string(self.0).unwrap().hash(state);
    }
}

use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_str,Value};

pub type Cost = NotNan<f64>;
pub const INFINITY: Cost = unsafe { NotNan::new_unchecked(std::f64::INFINITY) };

pub fn save_egraph_to_json(egraph: &EGraph<SimpleLanguage, ()>, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let json_rep = serde_json::to_string_pretty(&egraph).unwrap();
    fs::write(&file_path, json_rep)?;
    Ok(())
}

pub fn save_serialized_egraph_to_json(serialized_egraph: &SerializedEGraph, file_path: &PathBuf, root_ids: &[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(&file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &serialized_egraph)?;

    let root_eclasses_value: serde_json::Value = root_ids
        .iter()
        .map(|id| serde_json::Value::String(id.to_string()))
        .collect();

    let json_string = std::fs::read_to_string(&file_path)?;
    let mut json_data: serde_json::Value = serde_json::from_str(&json_string)?;
    json_data["root_eclasses"] = root_eclasses_value;

    let file = File::create(&file_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &json_data)?;

    Ok(())
}

pub fn egg_to_serialized_egraph<L, A>(egraph: &egg::EGraph<L, A>) -> egraph_serialize::EGraph
where
    L: Language + Display,
    A: Analysis<L>,
{
    use egraph_serialize::*;
    let mut out = EGraph::default();
    for class in egraph.classes() {
        for (i, node) in class.nodes.iter().enumerate() {
            out.add_node(
                format!("{}.{}", class.id, i),
                Node {
                    op: node.to_string(),
                    children: node
                        .children()
                        .iter()
                        .map(|id| NodeId::from(format!("{}.0", id)))
                        .collect(),
                    eclass: ClassId::from(format!("{}", class.id)),
                    cost: 1,
                },
            );
        }
    }
    out
}

pub struct WeightedAstSize;
impl CostFunction<SimpleLanguage> for WeightedAstSize {
    type Cost = usize;
    fn cost<C>(&mut self, enode: &SimpleLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let (mut unit) = match enode {
            SimpleLanguage::FST(_) => 100,
            SimpleLanguage::SND(_) => 100,
            SimpleLanguage::MAJ(_) => 100,
            SimpleLanguage::MAJI(_) => 100,
            SimpleLanguage::XOR3(_) => 100,
            _ => 0
        };

        enode.fold(unit, |sum, id| sum.saturating_add(costs(id)))
    }
}


pub fn process_json_prop_cost(json_str: &str) -> String {
    let mut data: Value = serde_json::from_str(&json_str).unwrap();

    if let Some(nodes) = data.get_mut("nodes").and_then(|nodes| nodes.as_object_mut()) {
        for node in nodes.values_mut() {
            let op = node["op"].as_str().unwrap();

            let ops = ["fa"];

            let new_cost = match op {
                op if ops.contains(&op) => 1,
                _ => 0,
            };
            
            node["cost"] = serde_json::to_value(new_cost).unwrap();
        }
    }

    serde_json::to_string_pretty(&data).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Node {
    op: String,
    children: Vec<String>,
    eclass: u32,
    cost: i16,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    nodes: IndexMap<String, Node>,
    root_eclasses: Vec<u32>,
    class_data: HashMap<String, Value>,
}

fn hash_children(children: &Vec<String>) -> String {
    let mut sorted_children = children.clone();
    sorted_children.sort();
    let json_str = serde_json::to_string(&sorted_children).unwrap();
    format!("{:x}", md5::compute(json_str))
}

fn remove_redundant_nodes(filename: &str) {
    let file_path: PathBuf = env::current_dir().unwrap().join(format!("json/serialized_egraph{}.json", filename));

    let file_content = fs::read_to_string(&file_path).expect("Unable to read file");
    let mut data: Data = serde_json::from_str(&file_content).expect("Unable to parse JSON");

    let mut eclass_hashes: HashMap<u32, HashSet<String>> = HashMap::new();
    let mut nodes_to_keep: IndexMap<String, Node> = IndexMap::new();
    let mut fa_nodes: HashMap<String, String> = HashMap::new(); // Map of node_id to children_hash for fa nodes
    let mut removed_fa_nodes: HashSet<String> = HashSet::new(); // Set of removed fa node IDs

    for (node_id, node) in data.nodes.iter() {
        let eclass = node.eclass;
        let children_hash = hash_children(&node.children);

        if node.op == "fa" {
            if let Some(existing_node_id) = fa_nodes.get(&children_hash) {
                removed_fa_nodes.insert(node_id.clone());
                continue;
            } else {
                fa_nodes.insert(children_hash.clone(), node_id.clone());
            }
        }

        let entry = eclass_hashes.entry(eclass).or_insert_with(HashSet::new);

        if !entry.contains(&children_hash) {
            entry.insert(children_hash);
            nodes_to_keep.insert(node_id.clone(), node.clone());
        }
    }

    // Remove fst and snd nodes that have removed fa nodes as children
    nodes_to_keep.retain(|node_id, node| {
        if node.op == "fst" || node.op == "snd" {
            for child in &node.children {
                if removed_fa_nodes.contains(child) {
                    return false;
                }
            }
        }
        true
    });

    data.nodes = nodes_to_keep;

    let new_file_content = serde_json::to_string_pretty(&data).expect("Unable to serialize JSON");
    fs::write(file_path, new_file_content).expect("Unable to write file");
}

fn save_json(filename:&str, egraph: EGraph<SimpleLanguage, ()>,root_ids:&[usize]) -> Result<(), Box<dyn std::error::Error>> {
    let output_egraph_json_path = env::current_dir().unwrap().join(format!("json/egraph{}.json",filename));
    save_egraph_to_json(&egraph, &output_egraph_json_path);

    let serialized_output_egraph = egg_to_serialized_egraph(&egraph);
    let json_string = serde_json::to_string(&serialized_output_egraph).unwrap();
    let cost_string = process_json_prop_cost(&json_string);
    let mut json_data: serde_json::Value = serde_json::from_str(&cost_string)?;
    json_data["root_eclasses"] = serde_json::Value::Array(root_ids.iter().map(|id| serde_json::Value::Number((*id as u32).into())).collect());
    let output_egraph_cost_json_path = env::current_dir().unwrap().join(format!("json/serialized_egraph{}.json",filename));
    let file = File::create(&output_egraph_cost_json_path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &json_data);
    remove_redundant_nodes(filename);
    Ok(())
}


fn build_expr_from_selection(egraph1: &EGraph<SimpleLanguage, ()>, result: &extraction_gym::ExtractionResult, egraph: &SerializedEGraph) -> Option<RecExpr<SimpleLanguage>> {
    let mut str_choice: IndexMap<String, &str> = IndexMap::default();
    for (classid, nodeid) in &result.choices {
        str_choice.insert(classid.as_ref().to_string(), nodeid.as_ref().split('.').last()?);
    }

    let mut id2node: IndexMap<Id, SimpleLanguage> = IndexMap::default();
    let mut root_node: Option<SimpleLanguage> = None;
    for class in egraph1.classes() {
        let class_id = class.id;
        let string_classid = class_id.to_string();
        let selected_id_str = match str_choice.get(&string_classid) {
            Some(id_str) => id_str,
            None => "0", // Return a default RecExpr if the selection fails
        };
        let num_id: usize = selected_id_str.parse().ok()?;
        let selected_node = class.nodes.get(num_id)?;
        id2node.insert(class_id, selected_node.clone());
        if get_node_type(selected_node) == "out" {
            root_node = Some(selected_node.clone());
        }
    }

    let root_node = root_node.expect("root_node was not initialized");
    let expr = root_node.build_recexpr(|id| id2node.get(&id).expect("Id not selected").clone());

    Some(expr)
}



fn main() -> io::Result<()> {

    let main_start = Instant::now();
    let args: Vec<String> = env::args().collect();

    // Create ./json directory if it doesn't exist
    let json_dir = Path::new("./json");
    if !json_dir.exists() {
        fs::create_dir_all(json_dir).expect("Failed to create ./json directory");
    }

    if args.len() != 3 {
        eprintln!("Usage: cargo run --features \"faster_greedy_dag_fa_mt\" --release -- benchmark/path/to/file.aig true|false");
        std::process::exit(1);
    }

    // Extract file path and check if it ends with .aig
    let aig_path = &args[1];
    if !aig_path.ends_with(".aig") {
        eprintln!("Input file must be an .aig file");
        std::process::exit(1);
    }

    // Parse the if_print argument
    let if_print: bool;
    if &args[2].to_lowercase() == "true" {
        if_print = true;
    } 
    else if &args[2].to_lowercase() == "false" {
        if_print = false;
    }
    else {
        eprintln!("Second argument must be 'true' or 'false'");
        std::process::exit(1);
    }

    // First, extract these values
    let path = Path::new(aig_path);
    let file_stem = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let parent_dir = path.parent().unwrap_or_else(|| Path::new("")).to_string_lossy().to_string();

    // When creating the benchmark name, clone file_stem to avoid moving it
    let benchmark = if parent_dir.is_empty() {
        file_stem.clone()
    } else {
        // Remove "benchmark/" prefix if it exists
        let parent_without_prefix = parent_dir.strip_prefix("benchmark/").unwrap_or(&parent_dir);
        if parent_without_prefix.is_empty() {
            file_stem.clone()
        } else {
            format!("{}/{}", parent_without_prefix, file_stem)
        }
    };

    // Create bench output path in the same directory as the aig file
    let bench_output_path = if parent_dir.is_empty() {
        format!("{}.bench", file_stem)
    } else {
        format!("{}/{}.bench", parent_dir, file_stem)
    };

    let abc_cmd = format!("./abc/abc -c \"read {}; write {}\"", aig_path, bench_output_path);

    // Execute ABC command
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(&abc_cmd)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .output()
        .expect("Failed to execute ABC command");

    if !output.status.success() {
        eprintln!("ABC command failed: {}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(1);
    }
    
    // Now use the converted .bench file
    let input_file_path1 = bench_output_path;
    // let (vertices, edges, in_pairs, out_pairs,edge_pairs,input_data,output_data) = lib::parse_graph_w_bench(&input_file_path1)?;
    // let input_file_path2=format!("{}{}_gia.aig", path, benchmark);
    match lib::parse_graph_w_bench(&input_file_path1) {
        Ok((vertices, _, in_pairs, _ ,edge_pairs, input, output)) => {
            let mut expr: RecExpr<SimpleLanguage> = RecExpr::default();
            let mut vertices_map: HashMap<String, Id> = HashMap::new();
            let mut queue: IndexSet<String> = vertices.clone();
            let mut out_id: HashMap<String, Id> = HashMap::new();
            queue.reverse();
            while !queue.is_empty(){
                let operation=queue.pop().unwrap();
                if let Some(input_edges) = in_pairs.get(&operation) {
                    let mut in_id = Vec::new();
                    for vertex in edge_pairs.get(input_edges).unwrap_or(&Vec::new()){
                        if let Some(id) = vertices_map.get(vertex) {
                            in_id.push(*id);
                        }
                    }
                    let name: Vec<&str> = input_edges.split("_").collect();
                    if name.len()==2 {
                        let stripped_name = name.first().unwrap().to_lowercase();
                        if let Some(value) = convert_to_simple_language_enum (in_id, &stripped_name) {
                            let temp_id = expr.add(value);
                            if output.contains(&operation){
                                out_id.insert(operation.clone(), temp_id);
                            }
                            vertices_map.insert(operation.clone(), temp_id);
                            // queue.retain(|x| x != &operation);
                        }
                        else {
                            panic!("Unknown enum variant for {}", stripped_name);
                        }
                    }
                    else {
                        panic!("Edge format error: {}", input_edges);
                    }
                } else {
                    let temp_id = expr.add(SimpleLanguage::Symbol(operation.clone().into()));
                    if output.contains(&operation){
                        out_id.insert(operation.clone(), temp_id);
                    }
                    vertices_map.insert(operation.clone(), temp_id);
                    // queue.retain(|x| x != &operation);
                }
            }
            // panic!("111");

            let mut vec_out_id: Vec<Id> = Vec::new();
            for element in &output{
                vec_out_id.push(out_id[element]);
            }

            let value = SimpleLanguage::OUT(vec_out_id);
            let temp_id = expr.add(value);
            vertices_map.insert("output".to_string(), temp_id);

            // println!("{}",expr);
            // writeln!(file, "{}", expr)?;
            if if_print {
                println!("Start E-graph Grownth");
            }
            let start = Instant::now();
            let runner: Runner<SimpleLanguage, ()>;
            let root: Id;
            // #[cfg(feature = "all_at_once_rule")]
            // {
            //     let runner1 = Runner::default().with_time_limit(Duration::from_secs(15000000000)).with_iter_limit(100000).with_node_limit(1000000).with_expr(&expr).run(&static_rules::make_rules());
            //     if if_print {
            //         println!(" Stop Reason: {:?}",runner1.stop_reason);
            //     }
            //     // writeln!(file,"Stop Reason: {:?}", runner1.stop_reason)?;
            //     let grownth_duration = start.elapsed();
            //     if if_print {
            //         println!("      Saturation: {:?}",grownth_duration);
            //     }
            //     // writeln!(file, "Saturation: {:?}",grownth_duration)?;
    
            //     // let egraph = runner1.egraph;
            //     let root_vec_usize: Vec<usize> = runner1.roots.iter().map(|&id| id.into()).collect();
            //     let root_usize: &[usize] = &root_vec_usize;
            //     save_json(runner1.egraph.clone(),root_usize);
            //     if if_print {
            //         println!("Json ready");
            //     }
            //     runner= runner1;
            //     root=runner.roots[0];
            // }
            // #[cfg(feature = "batched_rule")]
            let mut egg_egraph;
            {
                let l1 = 100;
                let l2 = 100;
                let pseudo_runner = Runner::default().with_expr(&expr).run(&static_rules::make_empty_rules());

                let runner1 = if benchmark.ends_with("_map") {
                    Runner::default()
                        .with_time_limit(Duration::from_secs(15000000000))
                        .with_iter_limit(l1)
                        .with_node_limit(10000000)
                        .with_expr(&expr)
                        .run(&rule1::make_rules1_map())
                } else {
                    Runner::default()
                        .with_time_limit(Duration::from_secs(15000000000))
                        .with_iter_limit(l1)
                        .with_node_limit(10000000)
                        .with_expr(&expr)
                        .run(&rule1::make_rules1())
                };

                let grownth_duration = start.elapsed();
                if if_print {
                    println!("Stop Reason1: {:?}",runner1.stop_reason);
                    println!("Saturation1: {:?}",grownth_duration);
                    println!("#nodes: {:?}\n",runner1.egraph.total_number_of_nodes());
                }

                let root_vec_usize: Vec<usize> = runner1.roots.iter().map(|&id| id.into()).collect();
                let root_usize: &[usize] = &root_vec_usize;

                let runner2 = if benchmark.ends_with("_map") {
                    Runner::default()
                        .with_time_limit(Duration::from_secs(15000000000))
                        .with_iter_limit(l2)
                        .with_node_limit(10000000)
                        .with_egraph(runner1.egraph)
                        .run(&static_rules::make_rules2_map())
                } else {
                    Runner::default()
                        .with_time_limit(Duration::from_secs(15000000000))
                        .with_iter_limit(l2)
                        .with_node_limit(10000000)
                        .with_egraph(runner1.egraph)
                        .run(&static_rules::make_rules2())
                };

                let grownth_duration = start.elapsed();
                if if_print {
                    println!("Stop Reason2: {:?}",runner2.stop_reason);
                    println!("Saturation2: {:?}",grownth_duration);
                    println!("#nodes: {:?}\n",runner2.egraph.total_number_of_nodes());
                }


                let runner3 = if benchmark.ends_with("_map") {
                    Runner::default()
                        .with_time_limit(Duration::from_secs(15000000000))
                        .with_iter_limit(l2)
                        .with_node_limit(10000000)
                        .with_egraph(runner2.egraph)
                        .run(&static_rules::make_rules3_map())
                } else {
                    Runner::default()
                        .with_time_limit(Duration::from_secs(15000000000))
                        .with_iter_limit(l2)
                        .with_node_limit(10000000)
                        .with_egraph(runner2.egraph)
                        .run(&static_rules::make_rules3())
                };
                let grownth_duration = start.elapsed();
                if if_print {
                    println!("Stop Reason3: {:?}",runner3.stop_reason);
                    println!("Saturation3: {:?}",grownth_duration);
                    println!("#nodes: {:?}\n",runner3.egraph.total_number_of_nodes());
                }

                egg_egraph = runner3.egraph.clone();
                let mut xor_node = Vec::new();
                let mut maj_node = Vec::new();
                for class in egg_egraph.classes() {
                    for node in &class.nodes {
                        match node {
                            SimpleLanguage::XOR3(_) => {
                                xor_node.push((class.id, node.clone()));
                            }
                            SimpleLanguage::MAJ(_) => {
                                maj_node.push((class.id, node.clone()));
                            }
                            _ => {}
                        }
                    }
                }
                for (class_id1, node1) in &xor_node {
                    for (class_id2, node2) in &maj_node {
                        if node1.children() == node2.children() {
                            let fa = egg_egraph.add(SimpleLanguage::FA(node1.children().to_vec()));
                            let fst = egg_egraph.add(SimpleLanguage::FST(vec![fa]));
                            let snd = egg_egraph.add(SimpleLanguage::SND(vec![fa]));
                            egg_egraph.union(*class_id1, fst);
                            egg_egraph.union(*class_id2, snd);
                        }
                    }
                }
                egg_egraph.rebuild();

                let grownth_duration = start.elapsed();
                if if_print {
                    println!("Total saturation: {:?}",grownth_duration);
                    println!("#nodes: {:?}\n",egg_egraph.total_number_of_nodes());
                }
                save_json("",egg_egraph.clone(),root_usize);
                println!("Json ready!");
            }

            // #[cfg(feature = "egg_extractor")]
            // {
            //     pub struct FASize;
            //     impl CostFunction<SimpleLanguage> for FASize {
            //         type Cost = i32;
            //         fn cost<C>(&mut self, enode: &SimpleLanguage, mut costs: C) -> Self::Cost
            //         where
            //             C: FnMut(Id) -> Self::Cost,
            //         {
            //             let node_size = match enode {
            //                 SimpleLanguage::FA(_) => -1,
            //                 _ => 0,
            //             };
            //             enode.fold(node_size, |sum, id| sum.saturating_add(costs(id)))
            //         }
            //     }

            //     let mut extractor = Extractor::new(&runner.egraph, FASize);
            //     let (best_cost, best) = extractor.find_best(root);
            //     println!("Number of matching pairs: {}", best_cost);
            // }


            #[cfg(feature = "global_greedy_dag")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let extractor_name: String = "global-greedy-dag".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "global-greedy-dag", grownth_duration, tree, dag);
            }

            #[cfg(feature = "faster_ilp_cbc")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let extractor_name: String = "faster-ilp-cbc".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                println!("{:<18}: runtime-{:?} dag-{}", "faster_ilp_cbc", grownth_duration, dag);
            }

            #[cfg(feature = "faster_greedy_dag")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let extractor_name: String = "faster-greedy-dag".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "faster-greedy-dag", grownth_duration, tree, dag);

                let expr: Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&egg_egraph, &result, &egraph);


                match expr {
                    Some(expr) => {
                        // println!("{}",expr);
                        // println!("{:?}",expr);
                        let pseudo_runner = Runner::default().with_expr(&expr).run(&static_rules::make_empty_rules());
                        // pseudo_runner.egraph.dot().to_pdf("foo.pdf").unwrap();
                        // pseudo_runner.egraph.dot().to_dot("foo.dot").unwrap();
                        let blif_path = &format!("output/{}/",benchmark);
                        let blif_dir = Path::new(&blif_path);
                        if !blif_dir.exists() {
                            if let Err(e) = fs::create_dir_all(blif_dir) {
                                eprintln!("Error creating directory {}: {}", blif_path, e);
                                std::process::exit(1);
                            }
                        }
                        if let Err(e) = lib::parse2bench(&blif_path, &benchmark, &expr,input,output) {
                            eprintln!("Failed to parse to BLIF: {}", e);
                        }
                    },
                    None => panic!("An error occurred, but no details are available."),
                }
            }

            #[cfg(feature = "faster_greedy_dag_fa")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let extractor_name: String = "faster-greedy-dag_fa".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                // println!("choices1: {:?}",result.choices);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                // println!("{}", dag);
                if if_print {
                    println!("{:<18}: runtime-{:?} dag:{}", "faster-greedy-dag_fa", grownth_duration, dag);
                }
                else{
                    println!("Rust: {} {}",input_file_path1, dag);
                }

                let expr: Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&egg_egraph, &result, &egraph);


                match expr {
                    Some(expr) => {
                        let pseudo_runner = Runner::default().with_expr(&expr).run(&static_rules::make_rules5());
                        let extractor = Extractor::new(&pseudo_runner.egraph, AstDepth);
                        let root = pseudo_runner.roots[0];
                        let (best_cost, best) = extractor.find_best(root);

                        let pseudo_runner = Runner::default().with_expr(&best).run(&static_rules::make_rules6());
                        let extractor = Extractor::new(&pseudo_runner.egraph, WeightedAstSize);
                        let root = pseudo_runner.roots[0];
                        let (best_cost, best) = extractor.find_best(root);


                        // pseudo_runner.egraph.dot().to_dot("foo.dot").unwrap();
                        let blif_path = &format!("output/{}/",benchmark);
                        let blif_dir = Path::new(&blif_path);
                        if !blif_dir.exists() {
                            if let Err(e) = fs::create_dir_all(blif_dir) {
                                eprintln!("Error creating directory {}: {}", blif_path, e);
                                std::process::exit(1);
                            }
                        }
                        if let Err(e) = lib::parse2bench(&blif_path, &benchmark, &best,input,output) {
                            eprintln!("Failed to parse to bench: {}", e);
                        }


                        let command = format!(
                            "./abc/abc -c \"cec {0} {1}\"",
                            format!("{}{}.bench", blif_path, benchmark), input_file_path1
                        );
                    
                        let output = Command::new("sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Failed to execute command");
                    
                        // 检查命令执行的结果
                        if !output.status.success() || !output.stderr.is_empty() {
                            eprintln!(
                                "Command execution failed with status: {}",
                                output.status
                            );
                            if !output.stderr.is_empty() {
                                eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                            }
                            std::process::exit(1);
                        }
                        if if_print {
                            println!("--------Compare the outputbench with the original bench--------");
                            println!("{}", String::from_utf8_lossy(&output.stdout));
                            if !output.stderr.is_empty() {
                                println!("{}", String::from_utf8_lossy(&output.stderr));
                            }
                        }
                    },
                    None => panic!("An error occurred, but no details are available."),
                }
            }

            #[cfg(feature = "faster_greedy_dag_fa_mt")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();

                let extractor_name: String = "faster-greedy-dag_fa_mt".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                // println!("choices1: {:?}",result.choices);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                // println!("{}", dag);
                if if_print {
                    println!("{:<18}: runtime-{:?} dag:{}", "faster-greedy-dag_fa", grownth_duration, dag);
                }
                else{
                    println!("Rust: {} {}",input_file_path1, dag);
                }


                let expr: RecExpr<SimpleLanguage> = build_expr_from_selection(&egg_egraph, &result, &egraph).unwrap();
                
                // Change from benchmark subfolder to direct output folder
                let blif_path = "output/";
                let blif_dir = Path::new(&blif_path);
                if !blif_dir.exists() {
                    if let Err(e) = fs::create_dir_all(blif_dir) {
                        eprintln!("Error creating directory {}: {}", blif_path, e);
                        std::process::exit(1);
                    }
                }

                // // Update file paths to use direct output folder
                // lib::parse2bench(&blif_path, &benchmark, &expr, input, output).unwrap();
                // let input_file = &format!("output/{}.bench", benchmark);

                // // Keep original bench output path but in direct output folder
                // let output_bench = &format!("output/{}_boole.bench", benchmark);
                // let output_aig = &format!("output/{}_boole.aig", benchmark);
                // println!("output_file: {}", output_aig);

                // Extract just the base name without path components
                let benchmark_base = benchmark.split('/').last().unwrap_or_else(|| &benchmark).to_string();
                
                // Update file paths to use direct output folder with base name only
                lib::parse2bench(&blif_path, &benchmark_base, &expr, input, output).unwrap();
                let input_file = &format!("output/{}.bench", benchmark_base);

                // Keep original bench output path but in direct output folder
                let output_bench = &format!("output/{}_boole.bench", benchmark_base);
                let output_aig = &format!("output/{}_boole.aig", benchmark_base);
                println!("output_file: {}", output_aig);

                // Process bench file as before
                lib::process_bench_file(input_file, &output_bench).unwrap();

                // Convert boole bench to AIG using ABC
                let abc_convert_cmd = format!(
                    "./abc/abc -c \"read {}; st; write {}\"",
                    output_bench, output_aig
                );

                let convert_output = Command::new("sh")
                    .arg("-c")
                    .arg(&abc_convert_cmd)
                    .output()
                    .expect("Failed to execute ABC conversion");

                if !convert_output.status.success() {
                    eprintln!(
                        "ABC conversion failed with status: {}",
                        convert_output.status
                    );
                    if !convert_output.stderr.is_empty() {
                        eprintln!("stderr: {}", String::from_utf8_lossy(&convert_output.stderr));
                    }
                    std::process::exit(1);
                }

                // Delete intermediate bench files
                let files_to_remove = vec![
                    format!("output/{}_boole.bench", benchmark_base),
                    format!("output/{}.bench", benchmark_base),
                    format!("benchmark/{}.bench", benchmark)
                ];
                
                for file in files_to_remove {
                    let _ = std::fs::remove_file(&file);
                }

                // Remove intermediate bench files
                // let files_to_remove = vec![
                //     format!("benchmark/{}.bench", benchmark),
                //     format!("output/{}.bench", benchmark),
                //     format!("output/{}_boole.bench", benchmark)
                // ];
                
                // for file in files_to_remove {
                //     if let Err(e) = std::fs::remove_file(&file) {
                //         eprintln!("Warning: Could not remove file {}: {}", file, e);
                //     }
                // }
                
                // Check if BLIF exists and convert from AIG if needed
                // let input_blif = format!("benchmark/{}.blif", benchmark);
                // if !Path::new(&input_blif).exists() {
                //     let convert_command = format!(
                //         "./abc/abc -c \"read benchmark/{}.aig; write benchmark/{}.blif\"",
                //         benchmark, benchmark
                //     );
                //     let convert_output = Command::new("sh")
                //         .arg("-c")
                //         .arg(&convert_command)
                //         .output()
                //         .expect("Failed to convert AIG to BLIF");

                //     if !convert_output.status.success() {
                //         eprintln!(
                //             "AIG to BLIF conversion failed with status: {}",
                //             convert_output.status
                //         );
                //         if !convert_output.stderr.is_empty() {
                //             eprintln!("stderr: {}", String::from_utf8_lossy(&convert_output.stderr));
                //         }
                //         std::process::exit(1);
                //     }
                // }

                // Now run CEC comparison
                // When referencing files, use benchmark_base instead of benchmark
                let command = format!(
                    "./abc/abc -c \"cec -n {0} {1}\"",
                    format!("benchmark/{}.aig", benchmark),  // Keep original path
                    format!("output/{}_boole.aig", benchmark_base)  // Use base name for output
                );

                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&command)
                    .output()
                    .expect("Failed to execute command");

                // Check command execution result
                if !output.status.success() || !output.stderr.is_empty() {
                    eprintln!(
                        "CEC comparison failed with status: {}",
                        output.status
                    );
                    if !output.stderr.is_empty() {
                        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                    }
                    std::process::exit(1);
                }

                if if_print {
                    println!("--------Compare the output BLIF with the original BLIF--------");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                    if !output.stderr.is_empty() {
                        println!("{}", String::from_utf8_lossy(&output.stderr));
                    }
                }
            }

            #[cfg(feature = "faster_bottom_up")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let extractor_name: String = "faster-bottom-up".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "faster-bottom-up", grownth_duration, tree, dag);
            }

            #[cfg(feature = "bottom_up")]
            {
                let mut extractors: indexmap::IndexMap<&str, extractor::ExtractorDetail, _> = extractor::extractors();
                extractors.retain(|_, ed| ed.get_use_for_bench());
                let filename: String = "json/serialized_egraph.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let extractor_name: String = "bottom-up".into();
                let ed = extractors
                    .get(extractor_name.as_str())
                    .with_context(|| format!("Unknown extractor: {extractor_name}"))
                    .unwrap();
                let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
                result.check(&egraph);
                let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                println!("{:<18}: runtime-{:?} tree-{} dag-{}", "bottom-up", grownth_duration, tree, dag);
            }
            #[cfg(feature = "read_from_file")]
            {
                let filename: String = "json/serialized_egraph1.json".into();
                let egraph = SerializedEGraph::from_json_file(&filename)
                .with_context(|| format!("Failed to parse {filename}"))
                .unwrap();
                let filename: String = "extract/aaa.txt".into();
                let file = File::open(&filename).expect("Unable to open file");
                let reader = BufReader::new(file);
                let mut result = extraction_gym::ExtractionResult::default();
                for line in reader.lines() {
                    let line = line.expect("Unable to read line");
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() == 2 {
                        let class_id_from_string: ClassId = String::from(parts[0]).into();
                        let node_id_from_string: NodeId = String::from(parts[1]).into();
                        result.choose(class_id_from_string, node_id_from_string);
                    }
                    else {
                        panic!("Invalid line: {}", line);
                    }
                }
                result.check(&egraph);
                // let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
                let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
                let grownth_duration = start.elapsed();
                println!("{:<18}: runtime-{:?} dag-{}", "bottom-up", grownth_duration, dag);
            }


            // #[cfg(feature = "self_extractor")]
            // {
            //     // Read the JSON file
                                
            //     // let filename: String = "json/serialized_egraph.json".into();
            //     // let egraph = SerializedEGraph::from_json_file(&filename)
            //     // .with_context(|| format!("Failed to parse {filename}"))
            //     // .unwrap();

            //     let file_content = fs::read_to_string("json/serialized_egraph.json")
            //         .expect("Unable to read file");
            //     let data: Value = serde_json::from_str(&file_content)
            //         .expect("Unable to parse JSON");

            //     let data_nodes = data["nodes"].as_object().expect("nodes should be an object");
            //     assert!(data["root_eclasses"].as_array().expect("root_eclasses should be an array").len() == 1);

            //     // let mut eclass: HashMap<String, Vec<String>> = HashMap::new();
            //     // for (node, node_data) in data_nodes {
            //     //     let node_eclass = node_data["eclass"].as_str().expect("eclass should be a string").to_string();
            //     //     eclass.entry(node_eclass).or_insert_with(Vec::new).push(node.clone());
            //     // }

            //     let mut xor_op = Vec::new();
            //     let mut maj_op = Vec::new();
            //     for (i, node_data) in data_nodes {
            //         let op = node_data["op"].as_str().expect("op should be a string");
            //         if op == "xor3" {
            //             xor_op.push(i.clone());
            //         } else if op == "maj" {
            //             maj_op.push(i.clone());
            //         }
            //     }

            //     let fa_pair = Arc::new(Mutex::new(HashSet::new()));
            //     let xor_op_children: HashMap<_, _> = xor_op.iter()
            //         .map(|op| (ChildrenHash(&data_nodes[op]["children"]), op))
            //         .collect();

            //     maj_op.par_iter().for_each(|op2| {
            //         let children2 = ChildrenHash(&data_nodes[op2]["children"]);
            //         if let Some(op1) = xor_op_children.get(&children2) {
            //             let mut fa_pair = fa_pair.lock().unwrap();
            //             fa_pair.insert(((*op1).clone(), op2.clone()));
            //         }
            //     });

            //     let fa_pair = Arc::try_unwrap(fa_pair).expect("Mutex still has multiple owners").into_inner().expect("Mutex cannot be locked");

            //     // Print the result
            //     // println!("{:?}", fa_pair);
            //     println!("Number of matching pairs: {}", fa_pair.len());
            // }



            // /*------------------faster-greedy-dag----------------*/
            // let start = Instant::now();
            // let extractor_name: String = "faster-greedy-dag".into();
            // let ed = extractors
            // .get(extractor_name.as_str())
            // .with_context(|| format!("Unknown extractor: {extractor_name}"))
            // .unwrap();
        
            // let result = ed.get_extractor().extract(&egraph, &egraph.root_eclasses);
            // // println!("{:?}",result.choices);
            // result.check(&egraph);

            // // println!("----------------------");

            // // let expr:Option<RecExpr<SimpleLanguage>> = build_expr_from_selection(&runner, &result);


            // // match expr {
            // //     Some(expr) => {
            // //         // println!("{}",expr);
            // //         // println!("{:?}",expr);
            // //         let pseudo_runner = Runner::default().with_expr(&expr).run(&static_rules::make_empty_rules());
            // //         // pseudo_runner.egraph.dot().to_pdf("foo.pdf").unwrap();
            // //         pseudo_runner.egraph.dot().to_dot("foo.dot").unwrap();
            // //     },
            // //     None => panic!("An error occurred, but no details are available."),
            // // }

            // println!("nodes={}",runner.egraph.total_number_of_nodes());
            // // writeln!(file, "nodes={}",runner.egraph.total_number_of_nodes())?;
            // let tree = result.tree_cost(&egraph, &egraph.root_eclasses);
            // let dag = result.dag_cost(&egraph, &egraph.root_eclasses);
            // let grownth_duration = start.elapsed();
            // println!("{:<18}: runtime:{:?} tree:{} dag:{}", "Faster-greedy-dag", grownth_duration, tree, dag);
            // // writeln!(file, "{:<18}: runtime:{:?} tree:{} dag:{}", "Faster-greedy-dag", grownth_duration, tree, dag)?;
            
        
            // // for (key, value) in result.choices.iter() {
            // //     println!("{}: {}", key, value);
            // // }

            // result.check(&egraph);
            
        
            
        },
        Err(e) => {
            // Handle the error here
            println!("Failed to parse the graph: {}", e);
        }
    }

    let main_duration = main_start.elapsed();
    // println!("Total time elapsed in main() is: {:?}", main_duration);
    // writeln!(file, "Total time is: {:?}", main_duration)?;

    // writeln!(file)?;


    Ok(())
}