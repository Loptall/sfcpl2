use super::{Direct, UnweightedListGraph};

pub trait Dfsable<'a, V, N>: Sized {
    fn dfs(&'a self, start: N) -> Dfs<'a, Self, V, N>;
}

pub struct Dfs<'a, G: Dfsable<'a, V, N>, V, N> {
    graph: &'a G,
    visited: V,
    stack: Vec<(N, Option<N>)>,
}

type Link<N> = (N, N);

impl<'a, D: Direct> Iterator for Dfs<'a, UnweightedListGraph<D>, Vec<bool>, usize> {
    type Item = Link<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((u, prev)) = self.stack.pop() {
            let visited = self.visited.clone();
            for &neighbor in self.graph.inner[u].iter().filter(|&&x| !visited[x]) {
                if !self.visited[neighbor] {
                    self.visited[neighbor] = true;
                    self.stack.push((neighbor, Some(u)));
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

impl<'a, D: Direct> Dfs<'a, UnweightedListGraph<D>, Vec<bool>, usize> {
    pub fn find<F: Fn(usize) -> bool>(&mut self, f: F) -> Option<usize> {
        for (_, to) in self {
            if f(to) {
                return Some(to);
            }
        }

        None
    }
}

// impl<'a, G: Dfsable<'a, V, N>, V, N> Dfs<'a, G, V, N> {}

impl<'a, D: Direct> Dfsable<'a, Vec<bool>, usize> for UnweightedListGraph<D> {
    fn dfs(&'a self, start: usize) -> Dfs<'a, Self, Vec<bool>, usize> {
        let mut visited = vec![false; self.len()];
        visited[start] = true;
        let mut stack = Vec::new();
        stack.push((start, None));
        Dfs {
            graph: self,
            visited,
            stack,
        }
    }
}
#[cfg(test)]
mod test {
    use super::super::UndirectedUnweightedListGraph;
    use super::Dfsable as _;
    #[test]
    fn dfs() {
        let edges = &[(0, 1), (1, 2), (1, 3), (3, 4)];
        let g = UndirectedUnweightedListGraph::from_edges(5, edges);
        let mut dfs = g.dfs(0);

        assert_eq!(dfs.next(), Some((0, 1)));
        assert_eq!(dfs.next(), Some((1, 3)));
        assert_eq!(dfs.next(), Some((3, 4)));
        assert_eq!(dfs.next(), Some((1, 2)));
    }
}
