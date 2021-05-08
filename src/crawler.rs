use std::{
    borrow::Borrow,
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub trait AdjacentNodes {
    type Node;

    fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node>;
}

pub struct Crawler<'a, G: AdjacentNodes> {
    graph: &'a G,
    visit: VecDeque<<G as AdjacentNodes>::Node>,
    visited: HashSet<<G as AdjacentNodes>::Node>,
}

impl<'a, G> Crawler<'a, G>
where
    G: AdjacentNodes,
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node>,
{
    pub fn new(graph: &'a G, start: <G as AdjacentNodes>::Node) -> Self {
        let mut visit = VecDeque::new();
        let visited = HashSet::new();

        visit.push_back(start);

        Self {
            graph,
            visit,
            visited,
        }
    }
}

impl<'a, G> Iterator for Crawler<'a, G>
where
    G: AdjacentNodes,
    <G as AdjacentNodes>::Node: Clone + Hash + Eq + Borrow<<G as AdjacentNodes>::Node>,
{
    type Item = <G as AdjacentNodes>::Node;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.visit.pop_front() {
            if self.visited.contains(&v) {
                continue;
            }

            for u in self.graph.adjacent_nodes(&v) {
                if !self.visited.contains(&u) {
                    self.visit.push_back(u);
                }
            }

            self.visited.insert(v.clone());
            return Some(v);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct AdjVec(Vec<Vec<usize>>);

    impl AdjacentNodes for AdjVec {
        type Node = usize;

        fn adjacent_nodes(&self, v: &Self::Node) -> Vec<Self::Node> {
            self.0.get(*v).cloned().unwrap_or(Vec::new())
        }
    }

    #[test]
    fn bfs() {
        let graph = AdjVec(vec![vec![1, 2], vec![0, 3], vec![3], vec![2, 0]]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<_> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 3]);
    }

    #[test]
    fn bfs2() {
        let graph = AdjVec(vec![vec![1], vec![0, 2, 4], vec![0, 3], vec![0], vec![0]]);

        let crawler = Crawler::new(&graph, 0);
        let nodes: Vec<_> = crawler.collect();

        assert_eq!(nodes, vec![0, 1, 2, 4, 3]);
    }
}
