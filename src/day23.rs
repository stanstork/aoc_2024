use crate::{utils::read_lines, AocDay};
use std::{
    collections::{HashMap, HashSet},
    vec,
};

pub struct AocDay23 {
    graph: HashMap<String, HashSet<String>>,
}

impl AocDay23 {
    pub fn new() -> Self {
        AocDay23 {
            graph: Self::build_graph(),
        }
    }

    pub fn part1(&self) -> usize {
        self.get_triplets()
            .iter()
            .filter(|t| t.iter().any(|s| s.starts_with('t')))
            .count()
    }

    pub fn part2(&self) -> String {
        let groups = self.get_connected_groups();
        let passwords = self.generate_passwords(&groups);
        passwords.iter().max_by_key(|p| p.len()).unwrap().clone()
    }

    fn build_graph() -> HashMap<String, HashSet<String>> {
        let lines = read_lines("input/day23.txt");
        let mut graph = HashMap::new();

        for line in lines {
            let parts: Vec<&str> = line.split('-').collect();
            let from = parts[0].to_string();
            let to = parts[1].to_string();

            graph
                .entry(from.clone())
                .or_insert(HashSet::new())
                .insert(to.clone());
            graph
                .entry(to.clone())
                .or_insert(HashSet::new())
                .insert(from.clone());
        }

        graph
    }

    fn get_triplets(&self) -> HashSet<Vec<String>> {
        let mut triplets = HashSet::new();
        for (node, neighbors) in &self.graph {
            let neighbors = neighbors.iter().collect::<Vec<_>>();
            for (i, a) in neighbors.iter().enumerate() {
                for b in &neighbors[i + 1..] {
                    if self.graph.get(*a).map_or(false, |n| n.contains(*b)) {
                        let mut triplet = vec![node.clone(), (*a).clone(), (*b).clone()];
                        triplet.sort();
                        triplets.insert(triplet);
                    }
                }
            }
        }
        triplets
    }

    fn get_connected_groups(&self) -> Vec<HashSet<String>> {
        let mut groups = vec![];
        for (node, neighbors) in &self.graph {
            let neighbors = neighbors.iter().collect::<Vec<_>>();
            let mut group = HashSet::new();

            for (i, a) in neighbors.iter().enumerate() {
                for b in &neighbors[i + 1..] {
                    if self.graph.get(*a).map_or(false, |n| n.contains(*b)) {
                        group.insert(node.clone());
                        group.insert((*a).clone());
                        group.insert((*b).clone());
                    }
                }
            }

            groups.push(group);
        }
        groups
    }

    fn is_fully_connected(&self, group: &HashSet<String>) -> bool {
        for a in group.iter() {
            for b in group.iter() {
                if a != b && !self.graph.get(a).unwrap().contains(b) {
                    return false;
                }
            }
        }
        true
    }

    fn generate_passwords(&self, groups: &Vec<HashSet<String>>) -> HashSet<String> {
        let mut passwords = HashSet::new();
        for group in groups.iter() {
            if self.is_fully_connected(group) {
                let mut password = group
                    .clone()
                    .iter()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>();

                password.sort();
                passwords.insert(password.join(","));
            }
        }
        passwords
    }
}

impl AocDay for AocDay23 {
    fn part1(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part1())
    }

    fn part2(&self) -> Box<dyn std::fmt::Display> {
        Box::new(self.part2())
    }
}
