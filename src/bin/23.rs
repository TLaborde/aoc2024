advent_of_code::solution!(23);


use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashSet, HashMap};

type Nodes = HashSet<u32>;
type Graph = HashMap<u32, Nodes>;
type Record = (u32, u32);

pub fn part_one(input: &str) -> Option<u32> {
    let (node_map_rev, edges) = parse_input(input);
    // make edges into a hashset
    let edges: HashSet<(u32, u32)> = edges.into_iter().collect();
    // for each edge...
    let mut good_sets = HashSet::new();
    for (from, to) in edges.iter() {
        // and each node
        for node in node_map_rev.keys() {
            // if neither from, to or not content the letter t, continue
            if !node_map_rev[from].starts_with("t") && !node_map_rev[to].starts_with("t") && !node_map_rev[node].starts_with("t") {
                continue;
            }
            if node != from && node != to && (edges.contains(&(*to, *node)) || edges.contains(&(*node, *to))) && (edges.contains(&(*node, *from)) || edges.contains(&(*from, *node))) {
                // sort from, to, node inside a collection
                let mut collections = vec![from.clone(), to.clone(), node.clone()];
                collections.sort();
                good_sets.insert(collections);
            }
        }
    }
    Some(good_sets.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let (node_map_rev, edges) = parse_input(input);

    let nodes = init_nodes(&edges);
    let r: Nodes = nodes.keys().copied().collect();
    let mut cliques: Vec<Nodes> = Vec::new();
    bron_kerbosh(&nodes, Nodes::new(), r, Nodes::new(), &mut cliques);

    let mut max_clique = Nodes::new();
    for cl in cliques.iter() {
        if cl.len() > max_clique.len() {
            max_clique = cl.clone();
        }
    }
    // sort max_clique and join with comma
    // use the node_map_rev to get the node name
    let mut max_clique_str = max_clique.iter().map(|x| node_map_rev[x].clone()).collect::<Vec<String>>();
    max_clique_str.sort();
    Some(max_clique_str.join(","))
}

fn parse_input(input: &str) -> (HashMap<u32, String>, Vec<(u32, u32)>) {
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split("-");
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        nodes.insert(from);
        nodes.insert(to);
    }
    // for each node, assign a number, make a hashmap of the node and the number
    let mut node_map = std::collections::HashMap::new();
    let mut node_map_rev = std::collections::HashMap::new();
    let mut i:u32 = 0;
    for node in nodes.iter() {
        node_map.insert(node.clone(), i);
        node_map_rev.insert(i, node.clone());
        i += 1;
    }
    let mut edges = Vec::new();
    for line in input.lines() {
        let mut parts = line.split("-");
        let from = parts.next().unwrap().to_string();
        let to = parts.next().unwrap().to_string();
        edges.push((node_map[from.as_str()], node_map[to.as_str()]));
    }
    (node_map_rev, edges)
}

fn init_nodes(records: &[Record]) -> Graph {
    let mut nodes: Graph = Graph::with_capacity(records.len());
    for r in records.iter() {
        let n: &mut Nodes = match nodes.entry(r.0) {
            Vacant(entry) => entry.insert(Nodes::new()),
            Occupied(entry) => entry.into_mut(),
        };
        n.insert(r.1);
        let n: &mut Nodes = match nodes.entry(r.1) {
            Vacant(entry) => entry.insert(Nodes::new()),
            Occupied(entry) => entry.into_mut(),
        };
        n.insert(r.0);
    }
    nodes.shrink_to_fit();
    nodes
}

fn bron_kerbosh(graph: &Graph, r: Nodes, mut p: Nodes, mut x: Nodes, cliques: &mut Vec<Nodes>) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else if !p.is_empty() {
        let nodes = p.iter().cloned().collect::<Nodes>();
        nodes.iter().for_each(|node| {
            let neighbours: &Nodes = graph.get(node).unwrap();
            let mut to_add: Nodes = Nodes::new();
            to_add.insert(*node);
            bron_kerbosh(
                graph,
                r.union(&to_add).cloned().collect(),
                p.intersection(&neighbours).cloned().collect(),
                x.intersection(&neighbours).cloned().collect(),
                cliques,
            );
            p.remove(node);
            x.insert(*node);
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
