pub mod bfs;
pub mod dfs;

use std::marker::PhantomData;

// pub trait Graph<N: Eq, E: Eq> {
//     fn nodes(&self) -> usize;
//     fn edges(&self) -> usize;
//     fn add_node(&mut self) -> N;
//     fn add_edge(&mut self, from: N, to: N) -> E;
//     fn get_nodes(&self, edge: E) -> (N, N);
//     fn get_edge(&self, from: N, to: N) -> Option<E>;
//     fn neibors_edges<'a>(&'a self, from: N) -> Vec<E>;
// }

pub trait Direct {
    const DIRECTED: bool;
}

pub struct Undirected;
pub struct Directed;

impl Direct for Directed {
    const DIRECTED: bool = true;
}
impl Direct for Undirected {
    const DIRECTED: bool = false;
}

pub trait Weigh<T> {
    const WEIGHTED: bool;
}

pub struct Unweighted;
pub struct Weighted<T> {
    phantom: PhantomData<T>,
}

impl Weigh<usize> for Unweighted {
    const WEIGHTED: bool = false;
}

impl<T> Weigh<T> for Weighted<T> {
    const WEIGHTED: bool = true;
}

pub struct ListGraph<T, D: Direct, W: Weigh<T>> {
    _phantom: (PhantomData<D>, PhantomData<W>),
    inner: Vec<Vec<T>>,
}

pub type UnweightedListGraph<D> = ListGraph<usize, D, Unweighted>;

pub type UndirectedUnweightedListGraph = UnweightedListGraph<Undirected>;
pub type DirectedUnweightedListGraph = UnweightedListGraph<Directed>;

pub type WeightedListGraph<T, D> = ListGraph<(usize, T), D, Weighted<T>>;

pub type UndirectedWeightedListGraph<T> = WeightedListGraph<T, Undirected>;
pub type DirectedWeightedListGraph<T> = WeightedListGraph<T, Directed>;

impl<D: Direct> UnweightedListGraph<D> {
    pub fn new(n: usize) -> Self {
        Self {
            _phantom: (PhantomData, PhantomData),
            inner: vec![Vec::new(); n],
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl UndirectedUnweightedListGraph {
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.inner[from].push(to);
        self.inner[to].push(from);
    }

    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut res = Self::new(n);
        for &(from, to) in edges {
            res.add_edge(from, to);
        }
        res
    }
}

impl DirectedUnweightedListGraph {
    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.inner[from].push(to);
    }

    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut res = Self::new(n);
        for &(from, to) in edges {
            res.add_edge(from, to);
        }
        res
    }
}

// pub struct MatGraph {
//     inner: Vec<Vec<Option<usize>>>,
// }

// pub struct GridGraoh<C> {
//     inner: Vec<Vec<C>>,
// }
