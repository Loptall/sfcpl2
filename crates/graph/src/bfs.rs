use std::collections::VecDeque;

use super::{Direct, UnweightedListGraph};

pub trait Bfsable<'a, V, N>: Sized {
    fn bfs(&'a self, start: N) -> Bfs<'a, Self, V, N>;
}

pub struct Bfs<'a, G: Bfsable<'a, V, N>, V, N> {
    graph: &'a G,
    visited: V,
    start: N,
    queue: VecDeque<(N, Option<N>)>,
}

type Link<N> = (N, N);

impl<'a, D: Direct> Iterator for Bfs<'a, UnweightedListGraph<D>, Vec<Option<usize>>, usize> {
    type Item = Link<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.queue.pop_front() {
            let visited = self.visited.clone();
            for &neighbor in self.graph.inner[u]
                .iter()
                .filter(|&&x| visited[x].is_none())
            {
                if self.visited[neighbor].is_none() {
                    self.visited[neighbor] = Some(self.visited[u].unwrap() + 1);
                    self.queue.push_back((neighbor, Some(u)));
                }
            }

            if let Some(prev) = prev {
                Some((prev, u))
            } else {
                self.next()
            }
        } else {
            None
        }
    }
}

impl<'a, D: Direct> Bfs<'a, UnweightedListGraph<D>, Vec<Option<usize>>, usize> {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn find<F: Fn(usize) -> bool>(&mut self, f: F) -> Option<usize> {
        for (_, to) in self {
            if f(to) {
                return Some(to);
            }
        }
        None
    }

    pub fn dist(&mut self, goal: usize) -> Option<usize> {
        if self.start == goal {
            return Some(0);
        }
        for (_, to) in self.into_iter() {
            if to == goal {
                return self.visited[goal];
            }
        }
        None
    }
}

// impl<'a, G: Bfsable<'a, V, N>, V, N> Bfs<'a, G, V, N> {}

impl<'a, D: Direct> Bfsable<'a, Vec<Option<usize>>, usize> for UnweightedListGraph<D> {
    fn bfs(&'a self, start: usize) -> Bfs<'a, Self, Vec<Option<usize>>, usize> {
        let mut visited = vec![None; self.len()];
        visited[start] = Some(0);
        let mut queue = VecDeque::new();
        queue.push_back((start, None));
        Bfs {
            graph: self,
            visited,
            start,
            queue,
        }
    }
}
#[cfg(test)]
mod test {
    use super::super::UndirectedUnweightedListGraph;
    use super::Bfsable as _;
    #[test]
    fn bfs() {
        let edges = &[(0, 1), (1, 2), (1, 3), (2, 4)];
        let g = UndirectedUnweightedListGraph::from_edges(5, edges);
        let mut bfs = g.bfs(0);

        assert_eq!(bfs.next(), Some((0, 1)));
        assert_eq!(bfs.next(), Some((1, 2)));
        assert_eq!(bfs.next(), Some((1, 3)));
        assert_eq!(bfs.next(), Some((2, 4)));
    }

    #[test]
    fn dist() {
        let edges = &[(0, 1), (1, 2), (1, 3), (2, 4)];
        let g = UndirectedUnweightedListGraph::from_edges(6, edges);

        assert_eq!(g.bfs(0).dist(0), Some(0));
        assert_eq!(g.bfs(0).dist(1), Some(1));
        assert_eq!(g.bfs(1).dist(0), Some(1));
        assert_eq!(g.bfs(0).dist(3), Some(2));
        assert_eq!(g.bfs(3).dist(0), Some(2));
        assert_eq!(g.bfs(0).dist(4), Some(3));
        assert_eq!(g.bfs(2).dist(3), Some(2));
        assert_eq!(g.bfs(2).dist(4), Some(1));
        assert_eq!(g.bfs(1).dist(5), None);
    }
}
