use std::fs;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct OrbitalMap {
    orbits: HashMap<String, HashSet<String>>,
}

impl OrbitalMap {
    pub fn new() -> OrbitalMap {
        OrbitalMap { orbits: HashMap::new() }
    }

    pub fn add(&mut self, orbitee: &str, orbiter: &str) {
        let list = self.orbits.entry(String::from(orbiter)).or_default();
        list.insert(String::from(orbitee));
    }

    pub fn count_direct(&self, orbitee: &str) -> u32 {
        let set = self.orbits.get(orbitee).unwrap();
        set.len() as u32
    }

    pub fn count_indirect(&self, orbitee: &str) -> u32 {
        self.count_indirect_impl(orbitee, 0) - 1
    }

    fn count_indirect_impl(&self, orbitee: &str, depth: u32) -> u32 {
        let set = self.orbits.get(orbitee);
        if set.is_none() {
            return 0;
        }

        let count: u32 = set
            .unwrap()
            .iter()
            .map(|o| {
                let c = self.count_indirect_impl(o, depth + 1);
                // println!("{} {} -> {} ({})", depth, orbitee, o, c);
                c
            })
            .sum();

        count + 1
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
    // let input = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN";
    let input = fs::read_to_string("input.txt").unwrap();

    let pairs = parse_input(&input);
    let mut orbits = OrbitalMap::new();

    for (orbitee, orbiter) in pairs {
        orbits.add(orbitee, orbiter);
    }

    println!("total: {}", orbits.count_all());
    // println!("distance: {}", orbits.distance_between("YOU", "SAN"));
}
