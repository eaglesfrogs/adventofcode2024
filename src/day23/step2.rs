use std::collections::{HashMap, HashSet};

pub fn bron_kerbosch(r: HashSet<String>, p: &mut HashSet<String>, x: &mut HashSet<String>, graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let mut cliques = Vec::<HashSet<String>>::new();

    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    }

    while p.is_empty() == false {
        let v = p.clone().into_iter().next().unwrap();

        let mut new_r = r.clone();
        new_r.insert(v.clone());

        let mut new_p = HashSet::<String>::new();
        let mut new_x = HashSet::<String>::new();
        let new_p_intersection = p.intersection(graph.get(&v).unwrap());
        for p in new_p_intersection {
            new_p.insert(p.clone());
        }
        let new_x_intersection = x.intersection(graph.get(&v).unwrap());
        for x in new_x_intersection {
            new_x.insert(x.clone());
        }

        let mut more_cliques = bron_kerbosch(new_r, &mut new_p, &mut new_x, graph);
        cliques.append(&mut more_cliques);

        p.remove(&v);
        x.insert(v.clone());
    }

    return cliques;
}

pub fn execute(data: &Vec<String>) {
    let mut network_map = HashMap::<String, HashSet<String>>::new();

    for line in data {
        let (pc1, pc2) = line.split_once("-").unwrap();

        let network = network_map.entry(String::from(pc1)).or_insert(HashSet::new());
        network.insert(String::from(pc2));

        let network = network_map.entry(String::from(pc2)).or_insert(HashSet::new());
        network.insert(String::from(pc1));
    }

    let mut vertices = HashSet::<String>::new();
    for key in network_map.keys() {
        vertices.insert(key.clone());
    }

    let results = bron_kerbosch(HashSet::new(), &mut vertices, &mut HashSet::new(), &network_map);

    let mut max_hashset = HashSet::<String>::new();
    for r in results {
        if r.len() > max_hashset.len() {
            max_hashset = r;
        }
    }

    let mut final_result = Vec::<String>::new();
    for s in max_hashset {
        final_result.push(s);
    }

    final_result.sort();

    println!("{:?}", final_result.join(","));
}
