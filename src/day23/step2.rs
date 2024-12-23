use std::collections::{HashMap, HashSet};

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
// https://www.geeksforgeeks.org/maximal-clique-problem-recursive-solution/
pub fn bron_kerbosch(current_clique: HashSet<String>, potential: &mut HashSet<String>, processed: &mut HashSet<String>, graph: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let mut cliques = Vec::<HashSet<String>>::new();

    if potential.is_empty() && processed.is_empty() {
        cliques.push(current_clique.clone());
    }

    while potential.is_empty() == false {
        let v = potential.clone().into_iter().next().unwrap();

        let mut new_current_clique = current_clique.clone();
        new_current_clique.insert(v.clone());

        let mut new_potential = HashSet::<String>::new();
        let mut new_processed = HashSet::<String>::new();
        let new_p_intersection = potential.intersection(graph.get(&v).unwrap());
        for p in new_p_intersection {
            new_potential.insert(p.clone());
        }
        let new_x_intersection = processed.intersection(graph.get(&v).unwrap());
        for x in new_x_intersection {
            new_processed.insert(x.clone());
        }

        let mut more_cliques = bron_kerbosch(new_current_clique, &mut new_potential, &mut new_processed, graph);
        cliques.append(&mut more_cliques);

        potential.remove(&v);
        processed.insert(v.clone());
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
