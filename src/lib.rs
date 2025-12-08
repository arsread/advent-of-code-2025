pub mod input;
pub mod runner;
pub mod days;

use im::{Vector, HashMap};


pub struct Day {
    pub number: u32,
    pub solve_1: fn(&str) -> String,
    pub solve_2: fn(&str) -> String,
}

#[derive(Clone, Debug)]
pub struct DSU {
    parent: Vector<usize>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        DSU { parent: (0..n).collect() }
    }

    pub fn find(self, x: usize) -> (usize, DSU) {
        let p = self.parent[x];
        if p == x { (x, self) } 
        else {
            let (root, new_dsu) = self.find(p);
            (root, DSU { parent: new_dsu.parent.update(x, root) })
        }
    }

    pub fn union(self, a: usize, b: usize) -> DSU {
        let (ra, d1) = self.find(a);
        let (rb, d2) = d1.find(b);
        if ra == rb { d2 } else { DSU { parent: d2.parent.update(ra, rb) } }
    }

    pub fn group_sizes(self) -> (HashMap<usize, usize>, DSU) {
        (0..self.parent.len())
            .fold(
                (HashMap::new(), self), 
                |(map, dsu), i| {
                    let (root, new_dsu) = dsu.find(i);
                    let count = map.get(&root).cloned().unwrap_or_default();
                    (map.update(root, count + 1), new_dsu)
        })
    }
}
