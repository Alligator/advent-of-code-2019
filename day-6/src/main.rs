use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct OrbitalMap {
    orbits: HashMap<String, String>,
}

impl OrbitalMap {
    pub fn new() -> OrbitalMap {
        OrbitalMap { orbits: HashMap::new() }
    }

    pub fn add(&mut self, orbitee: &str, orbiter: &str) {
        self.orbits.insert(String::from(orbiter), String::from(orbitee));
    }

    pub fn count_direct(&self, _orbitee: &str) -> u32 {
        // the input never has more than 1 direct orbit per thing, so, heh
        1
    }

    pub fn count_indirect(&self, orbitee: &str) -> u32 {
        self.count_indirect_impl(orbitee, 0) - 1
    }

    fn count_indirect_impl(&self, orbitee: &str, depth: u32) -> u32 {
        let other = self.orbits.get(orbitee);
        if other.is_none() {
            return 0;
        }

        self.count_indirect_impl(other.unwrap(), depth + 1) + 1
    }

    pub fn count_all(&self) -> u32 {
        self.orbits.keys()
            .map(|k| {
                let direct = self.count_direct(k);
                let indirect = self.count_indirect(k);
                // println!("{} direct: {} indirect: {}", k, direct, indirect);
                direct + indirect
            })
            .sum()
    }

    fn get_parents(&self, id: &str) -> HashMap<String, u32> {
        let mut parents = HashMap::new();
        let mut current_id = id;
        let mut dist = 1;

        while let Some(node) = self.orbits.get(current_id) {
            parents.insert(String::from(node), dist);
            current_id = node;
            dist += 1
        }

        parents
    }

    pub fn distance_between(&self, from: &str, to: &str) -> u32 {
        let from_parents = self.get_parents(from);
        let to_parents = self.get_parents(to);

        let mut smallest = 999999;
        for (k, dist) in from_parents {
            if let Some(other_dist) = to_parents.get(&k) {
                // println!("found common parent: {}", k);
                if dist + other_dist < smallest {
                    smallest = dist + other_dist;
                }
            }
        }
        
        smallest - 2
    }
}

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    input
        .split_whitespace()
        .map(|s| {
            let mut splits = s.splitn(2, ')');
            (splits.next().unwrap(), splits.next().unwrap())
        })
        .collect()
}

fn main() {
    // test input
    // let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    let input = fs::read_to_string("input.txt").unwrap();

    let pairs = parse_input(&input);
    let mut orbits = OrbitalMap::new();

    for (orbitee, orbiter) in pairs {
        orbits.add(orbitee, orbiter);
    }

    println!("total: {}", orbits.count_all());
    println!("distance: {}", orbits.distance_between("YOU", "SAN"));
}
