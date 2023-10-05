use std::collections::HashMap;
use std::cmp::{PartialEq, Eq};
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug)]
pub struct UnionFind<T> {
    groups: HashMap<T, T>,
}

impl<T> UnionFind<T>
where T: PartialEq + Eq + Hash + Copy + Debug + Clone
{
    pub fn new(elements: &[T]) -> Self {
        let mut groups: HashMap<T, T> = HashMap::new();
        elements.iter().for_each(|t: &T| {
            groups.insert(*t, *t);
        });
        UnionFind { groups }
    }

    pub fn union(self: &mut Self, t1: T, t2: T) {
        let p1 = self.parent(t1);
        let p2 = self.parent(t2);
        if p1 != p2 {
            self.groups.insert(p1, p2);
        }
    }

    pub fn parent(self: &mut Self, t: T) -> T {
        let mut reparent: Vec<T> = vec![];
        let mut curr: T = t;
        while let Some(parent) = self.groups.get(&curr) {
            // Root found
            if *parent == curr {
                // Apply path compression
                reparent.iter().for_each(|p: &T| {
                    self.groups.insert(*p, curr);
                });
                return curr;
            }
            else {
                reparent.push(curr);
                curr = *parent;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let values = vec![0, 1, 2, 3, 4, 5, 6];
        let mut uf = UnionFind::new(&values);
        uf.union(0, 1);
        uf.union(0, 2);
        uf.union(0, 3);
        uf.union(0, 4);
        for i in &[1, 2, 3, 4] {
            assert_eq!(uf.parent(0), uf.parent(*i));
        }
        assert_ne!(uf.parent(0), uf.parent(5));
        assert_ne!(uf.parent(0), uf.parent(6));
        assert_ne!(uf.parent(5), uf.parent(6));
    }

    #[test]
    fn test_2() {
        let values = vec![0, 1, 2, 3, 4, 5, 6];
        let mut uf = UnionFind::new(&values);
        uf.union(5, 6);
        uf.union(4, 5);
        uf.union(3, 4);
        uf.union(2, 3);
        uf.union(1, 2);
        uf.union(0, 1);
        println!("{:#?}", &uf.groups);
        for i in 0..values.len() - 1 {
            assert_eq!(uf.parent(i), uf.parent(i + 1));
        }
    }

}
