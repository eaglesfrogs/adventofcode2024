use std::collections::{HashMap, HashSet};

pub fn execute(data: &Vec<String>) {
    let mut network_map = HashMap::<String, Vec<String>>::new();
    let mut t_set = HashSet::<String>::new();

    for line in data {
        let (pc1, pc2) = line.split_once("-").unwrap();

        let network = network_map.entry(String::from(pc1)).or_insert(Vec::new());
        network.push(String::from(pc2));

        let network = network_map.entry(String::from(pc2)).or_insert(Vec::new());
        network.push(String::from(pc1));

        if pc1.starts_with("t") {
            t_set.insert(String::from(pc1));
        }

        if pc2.starts_with("t") {
            t_set.insert(String::from(pc2));
        }
    }

    let mut visited_sets = HashSet::<[String; 3]>::new();

    for t_node in t_set {
        for node_1 in network_map.get(&t_node).unwrap() {
            if *node_1 == t_node {
                continue;
            }

            for node_2 in network_map.get(node_1).unwrap() {
                if *node_2 == *node_1 || *node_2 == t_node {
                    continue;
                }

                if network_map.get(node_2).unwrap().contains(&t_node) {
                    let mut three_node_list = [t_node.clone(), node_1.clone(), node_2.clone()];
                    three_node_list.sort();

                    visited_sets.insert(three_node_list);
                }
            }
        }
    }

    println!("Step 1 Total {}", visited_sets.len());
}
