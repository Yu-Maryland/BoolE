use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::fs::File;
use egg::RecExpr;
use egg::Language;
use regex::Regex;
use std::io::{self, BufRead, BufReader, Write, Error,BufWriter};
use egraph_mapping::{SimpleLanguage,get_input_list};
use std::time::Instant;
use indexmap::IndexSet;
use egg::*;
use extraction_gym::*;
use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_str,Value};
use indexmap::IndexMap;

fn topological_sort_or_cycle(
    vertices: HashSet<String>, 
    Map_Edge_OutputNode: HashMap::<String, HashSet::<String>>, 
    out_pairs: HashMap<String, Vec<String>>
) -> Result<IndexSet<String>, Vec<String>> {
    let mut graph = HashMap::new();
    let mut in_degree = HashMap::new();
    let mut queue = VecDeque::new();
    let mut order = IndexSet::new();


    // 初始化所有顶点的入度为0
    for vertex in &vertices {
        in_degree.insert(vertex.clone(), 0);
    }

    // 构建图和计算入度
    for vertex in &vertices {
        // 找到由当前顶点产生的所有边，然后找到这些边连接的顶点
        if let Some(edges) = out_pairs.get(vertex) {
            for e in edges {
                // 查找由边e生成的顶点
                // let target_vertices = in_pairs.iter().filter_map(|(v, ed)| if ed == e { Some(v.clone()) } else { None }).collect::<Vec<_>>();
                if let Some(target_vertices) = Map_Edge_OutputNode.get(e){
                    for target_vertex in target_vertices {
                        graph.entry(vertex.clone()).or_insert_with(Vec::new).push(target_vertex.clone());
                        *in_degree.entry(target_vertex.clone()).or_insert(0) += 1;
                    }
                }
            }
        }
    }


    // 将所有入度为0的节点入队列
    for (vertex, deg) in &in_degree {
        if *deg == 0 {
            queue.push_back(vertex.clone());
        }
    }


    // 执行Kahn算法
    while let Some(vertex) = queue.pop_front() {
        order.insert(vertex.clone());
        if let Some(neighbors) = graph.get(&vertex) {
            for neighbor in neighbors {
                let degree = in_degree.get_mut(neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }
    // 检查是否所有的顶点都在排序中，如果不是则存在环
    if order.len() != vertices.len() {
        // 存在环
        return Err(find_cycle(vertices.clone(), Map_Edge_OutputNode, out_pairs));
    }

    Ok(order)
}


fn find_cycle(
    vertices: HashSet<String>, 
    Map_Edge_OutputNode: HashMap::<String, HashSet::<String>>, 
    out_pairs: HashMap<String, Vec<String>>
) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    let mut parent = HashMap::new();
    let mut graph = HashMap::new();

    // 构建图的映射
    for vertex in &vertices {
        if let Some(edges) = out_pairs.get(vertex) {
            for edge in edges {
                if let Some(next_vertices) = Map_Edge_OutputNode.get(edge){
                    for next_vertex in next_vertices {
                        graph.entry(vertex.clone()).or_insert_with(Vec::new).push(next_vertex.clone());
                    }
                }
                // if let Some(next_vertex) = in_pairs.iter().find(|&(_, e)| e == edge).map(|(v, _)| v) {
                    // graph.entry(vertex.clone()).or_insert_with(Vec::new).push(next_vertex.clone());
                // }
            }
        }
    }

    for vertex in &vertices {
        if !visited.contains(vertex) {
            if let Some(cycle) = dfs(vertex, &mut visited, &mut stack, &mut parent, &graph) {
                return cycle;
            }
        }
    }

    vec![]  // 如果没有找到环，返回空向量
}

fn dfs(
    vertex: &String, 
    visited: &mut HashSet<String>, 
    stack: &mut HashSet<String>,
    parent: &mut HashMap<String, String>,
    graph: &HashMap<String, Vec<String>>
) -> Option<Vec<String>> {
    visited.insert(vertex.clone());
    stack.insert(vertex.clone());

    if let Some(neighbors) = graph.get(vertex) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                parent.insert(neighbor.clone(), vertex.clone());
                if let Some(cycle) = dfs(neighbor, visited, stack, parent, graph) {
                    return Some(cycle);
                }
            } else if stack.contains(neighbor) {
                // 发现环
                return Some(construct_cycle(neighbor, vertex, parent));
            }
        }
    }

    stack.remove(vertex);
    None
}

fn construct_cycle(
    start: &String,
    current: &String,
    parent: &HashMap<String, String>
) -> Vec<String> {
    let mut cycle = Vec::new();
    let mut node = current;
    cycle.push(node.clone());
    while node != start {
        if let Some(p) = parent.get(node) {
            node = p;
            cycle.push(node.clone());
        }
    }
    cycle.push(start.clone()); // 完成环
    cycle.reverse();  // 反转使其从开始节点开始
    cycle
}

pub fn parse_graph_w_gia(input: &str) -> io::Result<(IndexSet<String>, HashSet<String>, HashMap<String, String>, HashMap<String, Vec<String>>, HashMap<String, Vec<String>>, HashSet<String>, Vec<String>)> {

    fn handle_not_operation(
        parts_int: &i32,
        vertices: &mut HashSet<String>,
        operation_index: &mut HashMap<String, usize>,
        edges: &mut HashSet<String>,
        in_pairs: &mut HashMap<String, String>,
        out_pairs: &mut HashMap<String, Vec<String>>,
        edge_pairs: &mut HashMap<String, Vec<String>>,
        Map_Edge_OutputNode: &mut HashMap<String, HashSet<String>>,
    ) {
        let output_node = "not_".to_owned() + &(parts_int / 2).to_string();
        let input_node = (parts_int / 2).to_string();
        let index = operation_index.get_mut("Not").expect("Operation 'Not' not found in operation_index");
        let edge_name = format!("{}_{}", "Not", index);
        *index += 1;
        edges.insert(edge_name.clone());
        vertices.insert(output_node.clone());
        in_pairs.insert(output_node.clone(), edge_name.clone());
        out_pairs.entry(input_node.clone())
            .or_insert_with(Vec::new)
            .push(edge_name.clone());
        edge_pairs.entry(edge_name.clone()).or_insert_with(Vec::new).push(input_node.clone());
        Map_Edge_OutputNode.entry(edge_name.clone())
            .or_insert(HashSet::new())
            .insert(output_node.clone());
    }

    
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut output_data: Vec<String> = Vec::new();
    let mut input_data: HashSet<String> = HashSet::new();
    let mut vertices: HashSet<String> = HashSet::new();
    let mut edges: HashSet<String> = HashSet::new();
    let mut operation_index: HashMap<String, usize> = {
        let mut map = HashMap::new();
        map.insert("And".to_owned(), 0);
        map.insert("Not".to_owned(), 0);
        map
    };
    let mut out_pairs: HashMap<String, Vec<String>> = HashMap::<String, Vec<String>>::new();
    let mut in_pairs: HashMap<String, String> = HashMap::<String, String>::new();
    let mut Map_Edge_OutputNode: HashMap<String, HashSet<String>> = HashMap::<String, HashSet::<String>>::new();
    let mut edge_pairs: HashMap<String, Vec<String>> = HashMap::<String, Vec<String>>::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<_> = line.split(" ").collect();

        if parts.iter().all(|part| part.parse::<i32>().is_ok()) {
            let parts_int: Vec<i32> = parts.iter().map(|part| part.parse::<i32>().unwrap()).collect();
            if parts_int.len() == 3 {
                let output_op = parts_int[0];
                assert!(output_op % 2 == 0, "The first part is not even");
                let op1 = parts_int[1];
                let op2 = parts_int[2];
                println!("output_op:{},op1:{},op2:{}",output_op,op1,op2);
                if !vertices.contains(&(op1/2).to_string()) {
                    input_data.insert((op1/2).to_string());
                    vertices.insert((op1/2).to_string());
                }
                if !vertices.contains(&(op2/2).to_string()) {
                    input_data.insert((op2/2).to_string());
                    vertices.insert((op2/2).to_string());
                }




                let mut input_node1;
                let mut input_node2;
                if op1 % 2  == 1 {
                    input_node1 = "not_".to_owned() + &(op1/2).to_string();
                    vertices.insert("not_".to_owned() + &(op1/2).to_string());
                    handle_not_operation(
                        &op1,
                        &mut vertices,
                        &mut operation_index,
                        &mut edges,
                        &mut in_pairs,
                        &mut out_pairs,
                        &mut edge_pairs,
                        &mut Map_Edge_OutputNode,
                    );
                }
                else{
                    input_node1 = (op1/2).to_string();
                }
                if op2 % 2  == 1 {
                    input_node2 = "not_".to_owned() + &(op2/2).to_string();
                    vertices.insert("not_".to_owned() + &(op2/2).to_string());
                    handle_not_operation(
                        &op2,
                        &mut vertices,
                        &mut operation_index,
                        &mut edges,
                        &mut in_pairs,
                        &mut out_pairs,
                        &mut edge_pairs,
                        &mut Map_Edge_OutputNode,
                    );
                }
                else{
                    input_node2 = (op2/2).to_string();
                }

                let output_node = (output_op/2).to_string();
                let index = operation_index.get_mut("And").expect("Operation 'And' not found in operation_index");
                let edge_name = format!("{}_{}", "And", index);
                *index += 1;
                edges.insert(edge_name.clone());
                vertices.insert(output_node.clone());
                in_pairs.insert(output_node.clone(), edge_name.clone());
                out_pairs.entry(input_node1.clone())
                    .or_insert_with(Vec::new)
                    .push(edge_name.clone());
                out_pairs.entry(input_node2.clone())
                    .or_insert_with(Vec::new)
                    .push(edge_name.clone());
                edge_pairs.entry(edge_name.clone()).or_insert_with(Vec::new).push(input_node1.clone());
                edge_pairs.entry(edge_name.clone()).or_insert_with(Vec::new).push(input_node2.clone());
                Map_Edge_OutputNode.entry(edge_name.clone())
                    .or_insert(HashSet::new())
                    .insert(output_node.clone());
            }
            else if parts_int.len() == 1 {
                if parts_int[0] % 2 == 0 {
                    output_data.push((parts_int[0]/2).to_string());
                }
                else {
                    output_data.push("not_".to_owned() + &(parts_int[0] / 2).to_string());
                    handle_not_operation(
                        &parts_int[0],
                        &mut vertices,
                        &mut operation_index,
                        &mut edges,
                        &mut in_pairs,
                        &mut out_pairs,
                        &mut edge_pairs,
                        &mut Map_Edge_OutputNode,
                    );
                }
            }
        }
    }
    
    // println!("vertices:{:?}",vertices);
    // println!("edges:{:?}",edges);
    // println!("operation_index:{:?}",operation_index);
    // println!("out_pairs:{:?}",out_pairs);
    // println!("in_pairs:{:?}",in_pairs);
    // println!("Map_Edge_OutputNode:{:?}",Map_Edge_OutputNode);
    // println!("edge_pairs:{:?}",edge_pairs);

    // panic!("Not implemented");
    match topological_sort_or_cycle(vertices.clone(), Map_Edge_OutputNode.clone(), out_pairs.clone()) {
        Ok(sorted) => {
            Ok((sorted, edges, in_pairs, out_pairs,edge_pairs,input_data,output_data))
        }
        Err(cycle) => {
            println!("Detected cycle: {:?}", cycle);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Cycle detected"))
        }
    }
}


pub fn parse_graph_w_bench(input: &str) -> io::Result<(IndexSet<String>, HashSet<String>, HashMap<String, String>, HashMap<String, Vec<String>>, HashMap<String, Vec<String>>, Vec<String>, Vec<String>)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut output_data: Vec<String> = Vec::new();
    let mut input_data: Vec<String> = Vec::new();
    let mut vertices: HashSet<String> = HashSet::new();
    let mut edges: HashSet<String> = HashSet::new();
    let mut operation_index: HashMap<String, usize> = HashMap::<String, usize>::new();
    let mut out_pairs: HashMap<String, Vec<String>> = HashMap::<String, Vec<String>>::new();
    let mut in_pairs: HashMap<String, String> = HashMap::<String, String>::new();
    let mut Map_Edge_OutputNode: HashMap<String, HashSet<String>> = HashMap::<String, HashSet::<String>>::new();
    let mut edge_pairs: HashMap<String, Vec<String>> = HashMap::<String, Vec<String>>::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("OUTPUT("){
            let parts: Vec<_> = line.split('(').collect();
            let var_name = parts[1].trim_end_matches(')');
            vertices.insert(var_name.to_string());
            if !output_data.contains(&var_name.to_string()) {
                output_data.push(var_name.to_string());
            }
            continue;
        }
        if line.starts_with("INPUT(") {
            let parts: Vec<_> = line.split('(').collect();
            let var_name = parts[1].trim_end_matches(')');
            vertices.insert(var_name.to_string());
            if !input_data.contains(&var_name.to_string()) {
                input_data.push(var_name.to_string());
            }
            continue;
        }

        let parts: Vec<_> = line.split(" = ").collect();
        if parts.len() == 2 {
            let left = parts[0].trim();
            let right = parts[1].trim();

            vertices.insert(left.to_string());
            

            let op_parts: Vec<_> = right.split('(').collect();
            if op_parts.len() == 2 {
                let operation = op_parts[0].trim();
                let operands = op_parts[1].trim().trim_end_matches(')').split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
                
                let index = operation_index.entry(operation.to_string()).or_insert(0);
                let edge_name = format!("{}_{}", operation.to_lowercase(), *index);
                edge_pairs.insert(edge_name.clone(),operands.clone());
                edges.insert(edge_name.clone());
                *index += 1;

                for operand in operands.iter() {
                    vertices.insert(operand.to_string());
                    out_pairs.entry(operand.clone()).or_insert_with(Vec::new).push(edge_name.clone());
                }
                in_pairs.insert(left.to_string(), edge_name.clone());
                Map_Edge_OutputNode.entry(edge_name.to_string())
                    .or_insert(HashSet::new())
                    .insert(left.to_string());
            }
        }
    }
    
    // println!("Map_Edge_OutputNode: {:?}", Map_Edge_OutputNode);
    match topological_sort_or_cycle(vertices.clone(), Map_Edge_OutputNode.clone(), out_pairs.clone()) {
        Ok(sorted) => {
            Ok((sorted, edges, in_pairs, out_pairs,edge_pairs,input_data,output_data))
        }
        Err(cycle) => {
            println!("Detected cycle: {:?}", cycle);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Cycle detected"))
        }
    }
}



pub fn parse_genlib(file_path: &str) -> Vec<(String, usize, usize, String)> {
    let mut cells = Vec::new();
    let mut current_cell: (_, usize, usize, _) = (String::new(), 100, 100, String::new());

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return cells; // Return empty vector if the file cannot be opened.
        }
    };

    let reader = io::BufReader::new(file);
    let gate_re = Regex::new(r"^GATE (\S+)\s+(\d+\.\d+)?\s+(.*);").unwrap();
    let delay_re = Regex::new(r"^DELAY \S+ \S+ (\d+\.\d+)").unwrap();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip lines that can't be read.
            }
        };

        if let Some(caps) = gate_re.captures(&line) {
            if !current_cell.0.is_empty() {
                cells.push(current_cell.clone());
            }
            current_cell = (
                caps[1].to_string(),
                (caps.get(2).map_or(1.0, |m| m.as_str().parse().unwrap_or(1.0)) as f64 * 100.0) as usize,
                100,
                caps[3].to_string(),
            );
        }

        if let Some(caps) = delay_re.captures(&line) {
            current_cell.2 = (caps[1].parse().unwrap_or(1) as f64 * 100.0) as usize;
        }
    }

    if !current_cell.0.is_empty() {
        cells.push(current_cell);
    }

    cells
}



// fn find_duplicates(output_vec: Vec<usize>, output_index: &HashMap<usize, String>) -> HashMap<usize, Vec<&String>> {
//     let mut index_map: HashMap<usize, Vec<&String>> = HashMap::new();

//     for (index, &value) in output_vec.iter().enumerate() {
//         if let Some(output_str) = output_index.get(&index) {
//             index_map.entry(value).or_insert(Vec::new()).push(output_str);
//         }
//     }

//     index_map.retain(|_, v| v.len() > 1);
//     index_map
// }

fn find_duplicates(output_vec: Vec<usize>, output_index: &Vec<String>) -> HashMap<usize, Vec<&String>> {
    let mut index_map: HashMap<usize, Vec<&String>> = HashMap::new();

    for (index, &value) in output_vec.iter().enumerate() {
        if let Some(output_str) = output_index.get(index) {
            index_map.entry(value).or_insert(Vec::new()).push(output_str);
        }
    }

    index_map.retain(|_, v| v.len() > 1);
    index_map
}



pub fn parse2blif(path: &str, benchmark: &str, expr: &RecExpr<SimpleLanguage>, input: Vec<String>, output: Vec<String>) -> Result<(), Error> {
    let model_name = &format!(".model {}_EGG\n", benchmark);
    let input = format!(".inputs {}\n", input.join(" "));
    let output_str = format!(".outputs {}\n", output.join(" "));

    let file_path=&format!("{}{}.blif", path, benchmark);

    let mut file = File::create(file_path)?;
    file.write_all(model_name.as_bytes())?;
    file.write_all(input.as_bytes())?;
    file.write_all(output_str.as_bytes())?;

    let mut node_index = HashMap::new();
    let mut temp_index=1;
    let mut output_vec = Vec::new();

    for node in expr.get_nodes(){
        match node{
            SimpleLanguage::OUT(_) => {
                for (_,input) in node.children().iter().enumerate(){
                    output_vec.push(usize::from(*input));
                }
            }
            _ => {}
        }
    }

    for (index, node) in expr.get_nodes().iter().enumerate(){
        match node{
            SimpleLanguage::Symbol(_) => {
                node_index.insert(index,format!("{}", node));
                if let Some(position) = output_vec.iter().position(|&x| x == index) {
                    if format!("{}",node) != format!("{}",output[position]){
                        let gate_line = format!(".gate HB1xp67_ASAP7_75t_L A={} Y={}\n", node, output[position]);
                        file.write_all(gate_line.as_bytes())?;
                    }
                }
            },
            SimpleLanguage::OUT(_) => {
            },
            _ => {
                let cell_input = get_input_list(format!("{}", node));
                if node.children().len()!=cell_input.len()-1{
                    panic!("Input length not match, {},{}",node.children().len(),cell_input.len());
                }
                // println!("{:?}",node.children());
                // println!("{:?}",(*node).children_mut());
                let mut gate_line = format!(".gate {} ", node);
                for (indice,input) in node.children().iter().enumerate(){
                    gate_line.push_str(cell_input[indice]);
                    gate_line.push_str("=");
                    // println!("{:?}",Id::from(*input));
                    // let _type_checker: usize = usize::from(*input);
                    let input_node = match node_index.get(&usize::from(*input)){
                        Some(node) => node,
                        None => panic!("Node not found for index {}", index),
                    };
                    gate_line.push_str(&input_node);
                    gate_line.push_str(" ");
                }
                gate_line.push_str(cell_input.last().unwrap());
                gate_line.push_str("=");
                // println!("4444-{:?}",expr.get_nodes()[1080]);
                // println!("3333-{:?}",output_vec);
                // println!("2222-{:?}",index);
                // if output_vec.contains(&index) {
                //     println!("1111-{:?}-{}-{}",output_size,output_vec.len(),index);
                //     output_size=output_size+1;
                // }
                
                match output_vec.iter().position(|&x| x == index) {
                    Some(pos) => {
                        gate_line.push_str(&format!("{}\n", output[pos]));
                        node_index.insert(index,format!("{}", output[pos]));
                    }
                    None => {                    
                        gate_line.push_str(&format!("temp_{}\n", temp_index));
                        node_index.insert(index,format!("temp_{}", temp_index));
                        temp_index=temp_index+1;
                    }
                }
                // if output_vec.contains(&index) {
                //     gate_line.push_str(&format!("{}\n", output[output_index]));
                //     node_index.insert(index,format!("{}", output[output_index]));
                //     output_index=output_index+1;
                // }
                // else{
                //     gate_line.push_str(&format!("temp_{}\n", temp_index));
                //     node_index.insert(index,format!("temp_{}", temp_index));
                //     temp_index=temp_index+1;
                // }

                file.write_all(gate_line.as_bytes())?;
            }
        }
    }

    let duplicates = find_duplicates(output_vec.clone(),&output);


    for (_, indices) in &duplicates {
        for i in 1..=indices.len()-1 {
            let gate_line = format!(".gate HB1xp67_ASAP7_75t_L A={} Y={}\n", indices[0], indices[i]);
            file.write_all(gate_line.as_bytes())?;
        }
    }



    file.write_all(".end\n".as_bytes())?;


    Ok(())
}


pub fn parse2bench(path: &str, benchmark: &str, expr: &RecExpr<SimpleLanguage>, input: Vec<String>, output: Vec<String>) -> Result<(), Error> {
    let mut file = File::create(&format!("{}{}.bench", path, benchmark))?;
    let mut node_index = HashMap::new();
    let mut temp_index=1;
    let mut output_vec = Vec::new();

    for i in input.iter() {
        file.write_all(format!("INPUT({})\n", i).as_bytes())?;
    }

    for i in output.iter() {
        file.write_all(format!("OUTPUT({})\n", i).as_bytes())?;
    }

    for node in expr.get_nodes(){
        match node{
            SimpleLanguage::OUT(_) => {
                for (_,input) in node.children().iter().enumerate(){
                    output_vec.push(usize::from(*input));
                }
            }
            _ => {}
        }
    }

    for (index, node) in expr.get_nodes().iter().enumerate(){
        match node {
            SimpleLanguage::Symbol(_) => {
                node_index.insert(index,format!("{}", node));
            }
            SimpleLanguage::OUT(_) => {}
            _ => {
                let mut gate_line = format!("");
                
                match output_vec.iter().position(|&x| x == index) {
                    Some(pos) => {
                        gate_line.push_str(&format!("{} = ", output[pos]));
                        node_index.insert(index,format!("{}", output[pos]));
                    }
                    None => {                    
                        gate_line.push_str(&format!("temp_{} = ", temp_index));
                        node_index.insert(index,format!("temp_{}", temp_index));
                        temp_index=temp_index+1;
                    }
                }

                gate_line.push_str(&format!("{}(", node).to_uppercase());


                for (indice,input) in node.children().iter().enumerate(){
                    let input_node = match node_index.get(&usize::from(*input)){
                        Some(node) => node,
                        None => panic!("Node not found for index {}", index),
                    };
                    gate_line.push_str(&input_node);
                    if indice != node.children().len()-1{
                        gate_line.push_str(", ");
                    }
                }

                gate_line.push_str(")\n");


                // println!("{:?}",gate_line);
                file.write_all(gate_line.as_bytes())?;
            }
        }
    }

    Ok(())
}


pub fn merge_sorted_vecs_unique(v1: Vec<usize>, v2: Vec<usize>) -> (Vec<usize>, HashMap<usize, usize>) {
    let mut result = Vec::with_capacity(v1.len() + v2.len());  // 预分配足够的空间
    let mut temp_counts = HashMap::new();  // 临时存储元素的出现次数
    let mut counts = HashMap::new();  // 最终只存储重复元素的计数
    let mut i = 0;  // v1的索引
    let mut j = 0;  // v2的索引

    // 当两个向量都还有元素时
    while i < v1.len() && j < v2.len() {
        let current = if v1[i] < v2[j] {
            let temp = v1[i];
            i += 1;
            temp
        } else {
            let temp = v2[j];
            j += 1;
            temp
        };

        let count = temp_counts.entry(current).or_insert(0);
        *count += 1;
        if *count == 1 {
            result.push(current);
        } else if *count == 2 {
            counts.insert(current, 2);
        } else {
            *counts.get_mut(&current).unwrap() += 1;
        }
    }

    // 处理任一向量中的剩余元素
    fn process_remaining_elements(vec: &[usize], i: &mut usize, temp_counts: &mut HashMap<usize, usize>, counts: &mut HashMap<usize, usize>, result: &mut Vec<usize>) {
        while *i < vec.len() {
            let current = vec[*i];
            let count = temp_counts.entry(current).or_insert(0);
            *count += 1;
            if *count == 1 {
                result.push(current);
            } else if *count == 2 {
                counts.insert(current, 2);
            } else {
                *counts.get_mut(&current).unwrap() += 1;
            }
            *i += 1;
        }
    }

    process_remaining_elements(&v1, &mut i, &mut temp_counts, &mut counts, &mut result);
    process_remaining_elements(&v2, &mut j, &mut temp_counts, &mut counts, &mut result);

    (result, counts)
}



pub fn merge_and_update_maps(map1: &mut HashMap<usize, usize>, map2: &HashMap<usize, usize>) {
    // 遍历第二个哈希图
    for (key, _) in map2 {
        // 检查这个键是否也在第一个哈希图中
        match map1.get_mut(key) {
            Some(value1) => {
                // 如果在两个哈希图中都有，将它们的值相加并更新第一个哈希图
                *value1 += 1;
            },
            None => {
                // 如果在第一个哈希图中没有找到该键，将其从第二个哈希图中插入到第一个哈希图中
                map1.insert(*key, 2);
            }
        }
    }
}

pub fn insert_sorted(vec: &mut Vec<usize>, value: usize, duplicates: &mut HashMap<usize, usize>) {
    match vec.binary_search(&value) {
        Ok(_) => {
            // 如果找到相同的值，增加计数，如果不存在则初始化为 2
            *duplicates.entry(value).or_insert(1) += 1;
        },
        Err(pos) => {
            // 如果没找到，插入该值到适当位置
            vec.insert(pos, value);
        },
    }
}




pub fn process_bench_file(input_file: &str, output_file: &str) -> Result<(), io::Error> {
    // Extract benchmark name from input file path
    let benchmark_name = std::path::Path::new(input_file)
    .file_stem()
    .and_then(|s| s.to_str())
    .unwrap_or("Circuit");

    // Read lines from input file
    let lines = read_lines(input_file)?;
    
    // Build node_map
    let re = Regex::new(r"(\w+) = (\w+)\((.+)\)").unwrap();
    let mut node_map: HashMap<String, (String, String)> = HashMap::new();
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    
    // First pass to collect inputs/outputs
    for line in &lines {
        let line = line.trim();
        if line.starts_with("INPUT(") {
            let input = line.trim_start_matches("INPUT(").trim_end_matches(")");
            inputs.push(input.to_string());
        } else if line.starts_with("OUTPUT(") {
            let output = line.trim_start_matches("OUTPUT(").trim_end_matches(")");
            outputs.push(output.to_string());
        } else if let Some(caps) = re.captures(line) {
            let node = caps.get(1).unwrap().as_str().to_string();
            let operation = caps.get(2).unwrap().as_str().to_string();
            let operands = caps.get(3).unwrap().as_str().to_string();
            node_map.insert(node, (operation, operands));
        }
    }

    // Generate BLIF file
    let blif_path = output_file.replace(".bench", ".blif");
    let mut blif_content = generate_blif_file(&lines, &inputs, &outputs, &node_map, benchmark_name)?;
    
    // Add FA subcircuit definition
    blif_content.push_str("\n.model FA\n");
    blif_content.push_str(".inputs a b cin\n");
    blif_content.push_str(".outputs s cout\n");
    blif_content.push_str(".names a b and1\n11 1\n");
    blif_content.push_str(".names a b and1_\n00 1\n");
    blif_content.push_str(".names and1 and1_ xor\n00 1\n");
    blif_content.push_str(".names cin xor and2\n11 1\n");
    blif_content.push_str(".names cin xor and2_\n00 1\n");
    blif_content.push_str(".names and2 and2_ s\n00 1\n");
    blif_content.push_str(".names and1 and2 cout\n00 0\n");
    blif_content.push_str(".end\n");

    // Write BLIF file
    let mut blif_file = File::create(&blif_path)?;
    blif_file.write_all(blif_content.as_bytes())?;

    // Process bench file lines
    let mut new_lines: Vec<String> = Vec::new();
    for line in &lines {
        let line = line.trim();
        if let Some(caps) = re.captures(line) {
            let node = caps.get(1).unwrap().as_str();
            let operation = caps.get(2).unwrap().as_str();
            let operand = caps.get(3).unwrap().as_str();
            if operation == "SND" && node_map.contains_key(operand) && node_map[operand].0 == "FA" {
                let new_line = format!("{} = MAJ({})\n", node, node_map[operand].1);
                new_lines.push(new_line);
            } else if operation == "FST" && node_map.contains_key(operand) && node_map[operand].0 == "FA" {
                let new_line = format!("{} = XOR3({})\n", node, node_map[operand].1);
                new_lines.push(new_line);
            } else {
                new_lines.push(line.to_string() + "\n");
            }
        } else {
            new_lines.push(line.to_string() + "\n");
        }
    }

    // Handle FA subcircuits
    let re_fa = Regex::new(r"(\w+) = FA\((.+)\)").unwrap();
    let mut subckt_lines: Vec<String> = Vec::new();
    let mut fa_counter = 0;
    
    // First pass: collect subckt lines with proper numbering
    for line in &new_lines {
        if let Some(caps) = re_fa.captures(line.trim()) {
            let node = caps.get(1).unwrap().as_str();
            let operands = caps.get(2).unwrap().as_str();
            let operand_list: Vec<&str> = operands.split(", ").collect();
            if operand_list.len() == 3 {
                let mut xor_node = String::new();
                let mut maj_node = String::new();
                for (key, (op, ref op_node)) in &node_map {
                    if op == "FST" && op_node == node {
                        xor_node = key.clone();
                    } else if op == "SND" && op_node == node {
                        maj_node = key.clone();
                    }
                }
                let subckt = format!("# subckt FA a={} b={} cin={} s={} cout={}\n",
                    operand_list[0], operand_list[1], operand_list[2], xor_node, maj_node);
                subckt_lines.push(subckt);
                fa_counter += 1;
            }
        }
    }

    // Combine subckt lines with filtered lines
    let mut final_lines: Vec<String> = subckt_lines;
    final_lines.extend(new_lines.into_iter()
        .filter(|line| !re_fa.is_match(line.trim())));

    // Flatten MAJ and XOR3 gates
    let flattened_lines = flatten_gates(final_lines, &node_map);

    // Write to bench output file
    let mut output = File::create(output_file)?;
    for line in flattened_lines {
        output.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn generate_blif_file(
    lines: &Vec<String>,
    inputs: &Vec<String>, 
    outputs: &Vec<String>,
    node_map: &HashMap<String, (String, String)>,
    benchmark_name: &str,
) -> Result<String, io::Error> {
    let mut blif_content = String::new();
    
    // Add model name
    blif_content.push_str(&format!(".model {}\n", benchmark_name));
    
    // Add inputs
    blif_content.push_str(".inputs ");
    blif_content.push_str(&inputs.join(" "));
    blif_content.push_str("\n");
    
    // Add outputs 
    blif_content.push_str(".outputs ");
    blif_content.push_str(&outputs.join(" "));
    blif_content.push_str("\n");

    // Process gates
    let re = Regex::new(r"(\w+) = (\w+)\((.+)\)").unwrap();
    for line in lines {
        let line = line.trim();
        if let Some(caps) = re.captures(line) {
            let node = caps.get(1).unwrap().as_str();
            let operation = caps.get(2).unwrap().as_str();
            let operands = caps.get(3).unwrap().as_str();
            
            match operation {
                "AND" => {
                    blif_content.push_str(&format!(".names {} {}\n", operands.replace(", ", " "), node));
                    blif_content.push_str("11 1\n");
                },
                "OR" => {
                    blif_content.push_str(&format!(".names {} {}\n", operands.replace(", ", " "), node));
                    blif_content.push_str("1- 1\n-1 1\n");
                },
                "NOT" => {
                    blif_content.push_str(&format!(".names {} {}\n", operands, node));
                    blif_content.push_str("0 1\n");
                },
                "XOR3" => {
                    let operand_list: Vec<&str> = operands.split(", ").collect();
                    if operand_list.len() == 3 {
                        // XOR3 implementation using .names
                        let (a, b, c) = (operand_list[0], operand_list[1], operand_list[2]);
                        
                        // Intermediate nodes
                        blif_content.push_str(&format!(".names {} {} {}_t1\n", a, b, node));
                        blif_content.push_str("10 1\n01 1\n");
                        
                        blif_content.push_str(&format!(".names {}_t1 {} {}\n", node, c, node));
                        blif_content.push_str("10 1\n01 1\n");
                    }
                },
                "MAJ" => {
                    let operand_list: Vec<&str> = operands.split(", ").collect();
                    if operand_list.len() == 3 {
                        // Majority gate implementation using .names
                        let (a, b, c) = (operand_list[0], operand_list[1], operand_list[2]);
                        
                        blif_content.push_str(&format!(".names {} {} {} {}\n", a, b, c, node));
                        blif_content.push_str("11- 1\n1-1 1\n-11 1\n");
                    }
                },
                "FA" => {
                    let operand_list: Vec<&str> = operands.split(", ").collect();
                    if operand_list.len() == 3 {
                        let mut xor_node = String::new();
                        let mut maj_node = String::new();
                        for (key, (op, ref op_node)) in node_map {
                            if op == "FST" && op_node == node {
                                xor_node = key.clone();
                            } else if op == "SND" && op_node == node {
                                maj_node = key.clone();
                            }
                        }
                        blif_content.push_str(&format!(".subckt FA a={} b={} cin={} s={} cout={}\n",
                            operand_list[0], operand_list[1], operand_list[2], xor_node, maj_node));
                    }
                },
                _ => {} // Skip unknown gates
            }
        }
    }
    
    blif_content.push_str(".end\n");
    Ok(blif_content)
}

fn read_lines<P>(filename: P) -> Result<Vec<String>, io::Error>
where P: AsRef<std::path::Path>, {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn flatten_gates(lines: Vec<String>, node_map: &HashMap<String, (String, String)>) -> Vec<String> {
    // Define bench templates
    let maj_bench = "
e = NOT(x)
f = NOT(z)
g = AND(x, z)
h = NOT(g)
i = AND(e, f)
j = NOT(i)
l = AND(h, j)
m = AND(y, l)
n = NOT(m)
o = AND(h, n)
d = NOT(o)
";
    let xor_bench = "
e = NOT(x)
p = NOT(y)
f = NOT(z)
g = AND(x, z)
h = NOT(g)
i = AND(e, f)
j = NOT(i)
l = AND(h, j)
q = NOT(l)
m = AND(y, l)
n = NOT(m)
r = AND(p, q)
s = NOT(r)
d = AND(n, s)
";
    let xor_used_bench = "
p = NOT(y)
q = NOT(l)
r = AND(p, q)
s = NOT(r)
d = AND(n, s)
";

    // Replace gates
    let mut flattened_lines: Vec<String> = Vec::new();
    for line in lines {
        let new_lines = flatten_line(&line, node_map, maj_bench, xor_bench, xor_used_bench);
        flattened_lines.extend(new_lines);
    }
    flattened_lines
}

fn flatten_line(
    line: &str,
    node_map: &HashMap<String, (String, String)>,
    maj_bench: &str,
    xor_bench: &str,
    xor_used_bench: &str,
) -> Vec<String> {
    if line.contains("MAJ") {
        return replace_gate(line, "MAJ", maj_bench, node_map, xor_used_bench);
    } else if line.contains("XOR3") {
        let re = Regex::new(r"(\w+) = XOR3\((.+)\)").unwrap();
        if let Some(caps) = re.captures(line.trim()) {
            let node = caps.get(1).unwrap().as_str();
            let operands = caps.get(2).unwrap().as_str();
            let operand_list: Vec<&str> = operands.split(", ").collect();
            if operand_list.len() != 3 {
                return vec![line.to_string()];
            }
            let (a, b, c) = (operand_list[0], operand_list[1], operand_list[2]);
            if let Some((op, fst_operand)) = node_map.get(node) {
                if op == "FST" {
                    if let Some((fa_op, _)) = node_map.get(fst_operand) {
                        if fa_op == "FA" {
                            // Find the corresponding SND node
                            let mut snd_node = None;
                            for (key, value) in node_map {
                                if value.0 == "SND" && value.1 == *fst_operand {
                                    snd_node = Some(key.clone());
                                    break;
                                }
                            }
                            if let Some(snd_node) = snd_node {
                                // Use xor_used_bench
                                let bench_lines: Vec<&str> = xor_used_bench.trim().lines().collect();
                                let mut replaced_lines: Vec<String> = Vec::new();
                                for bench_line in bench_lines {
                                    let bench_line = bench_line.trim();
                                    if !bench_line.is_empty() {
                                        let mut bench_line = bench_line.replace("x", a).replace("y", b).replace("z", c);
                                        bench_line = bench_line.replace("d", node);
                                        bench_line = bench_line.replace("l", &format!("{}_l", snd_node)).replace("n", &format!("{}_n", snd_node));
                                        // Prefix internal nodes with the node name
                                        let internal_nodes = ["e", "f", "g", "h", "i", "j", "l", "m", "n", "o", "p", "q", "r", "s"];
                                        for &internal_node in &internal_nodes {
                                            let re_internal = Regex::new(&format!(r"\b{}\b", internal_node)).unwrap();
                                            bench_line = re_internal.replace_all(&bench_line, format!("{}_{}", node, internal_node)).to_string();
                                        }
                                        replaced_lines.push(bench_line + "\n");
                                    }
                                }
                                return replaced_lines;
                            }
                        }
                    }
                }
            }
            // Use xor_bench
            return replace_gate(line, "XOR3", xor_bench, node_map, xor_used_bench);
        }
    }
    vec![line.to_string()]
}

fn replace_gate(
    line: &str,
    gate_type: &str,
    bench_content: &str,
    node_map: &HashMap<String, (String, String)>,
    xor_used_bench: &str,
) -> Vec<String> {
    let re = Regex::new(&format!(r"(\w+) = {}\((.+)\)", gate_type)).unwrap();
    if let Some(caps) = re.captures(line.trim()) {
        let node = caps.get(1).unwrap().as_str();
        let operands = caps.get(2).unwrap().as_str();
        let operand_list: Vec<&str> = operands.split(", ").collect();
        if operand_list.len() != 3 {
            return vec![line.to_string()];
        }
        let (a, b, c) = (operand_list[0], operand_list[1], operand_list[2]);
        // Process bench_content
        let bench_lines: Vec<&str> = bench_content.trim().lines().collect();
        let mut replaced_lines: Vec<String> = Vec::new();
        for bench_line in bench_lines {
            let bench_line = bench_line.trim();
            if !bench_line.is_empty() {
                let mut bench_line = bench_line.replace("x", a).replace("y", b).replace("z", c);
                bench_line = bench_line.replace("d", node);
                // Prefix internal nodes with the node name
                let internal_nodes = ["e", "f", "g", "h", "i", "j", "l", "m", "n", "o", "p", "q", "r", "s"];
                for &internal_node in &internal_nodes {
                    let re_internal = Regex::new(&format!(r"\b{}\b", internal_node)).unwrap();
                    bench_line = re_internal.replace_all(&bench_line, format!("{}_{}", node, internal_node)).to_string();
                }
                replaced_lines.push(bench_line + "\n");
            }
        }
        return replaced_lines;
    }
    vec![line.to_string()]
}