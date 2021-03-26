use std::collections::VecDeque;

use super::{Direct, UnweightedListGraph};

pub trait Bfsable<'a, V, N>: Sized {
    fn bfs(&'a self, start: N) -> Bfs<'a, Self, V, N>;
}

pub struct Bfs<'a, G: Bfsable<'a, V, N>, V, N> {
    graph: &'a G,
    visited: V,
    queue: VecDeque<(N, Option<N>)>,
}

type Path<N> = (N, N);

impl<'a, D: Direct> Iterator for Bfs<'a, UnweightedListGraph<D>, Vec<bool>, usize> {
    type Item = Path<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.queue.pop_front() {
            let visited = self.visited.clone();
            for &neighbor in self.graph.inner[u].iter().filter(|&&x| !visited[x]) {
                if !self.visited[neighbor] {
                    self.visited[neighbor] = true;
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

impl<'a, D: Direct> Bfs<'a, UnweightedListGraph<D>, Vec<bool>, usize> {
    pub fn find<F: Fn(usize) -> bool>(&mut self, f: F) -> Option<usize> {
        for (_, to) in self {
            if f(to) {
                return Some(to);
            }
        }
        None
    }
}

// impl<'a, G: Bfsable<'a, V, N>, V, N> Bfs<'a, G, V, N> {}

impl<'a, D: Direct> Bfsable<'a, Vec<bool>, usize> for UnweightedListGraph<D> {
    fn bfs(&'a self, start: usize) -> Bfs<'a, Self, Vec<bool>, usize> {
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        let mut queue = VecDeque::new();
        queue.push_back((start, None));
        Bfs {
            graph: self,
            visited,
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
}
