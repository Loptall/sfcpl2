use std::{cmp::Reverse, collections::BinaryHeap, ops::Add};

use num_traits::Zero;

use crate::{Direct, Weigh, Weighted, WeightedListGraph};

pub trait Dijkstrable<'a, S, D>: Sized
where
    S: Clone + Add + Ord,
    Weighted<S>: Weigh<(usize, S)>,
    D: Direct,
{
    fn dijkstra(&'a self, start: usize) -> Dijkstra<'a, S, D>;
}

pub struct Dijkstra<'a, S, D>
where
    S: Clone + Add + Ord,
    Weighted<S>: Weigh<(usize, S)>,
    D: Direct,
{
    graph: &'a WeightedListGraph<S, D>,
    distance: Vec<Option<S>>,
    start: usize,
    p_queue: BinaryHeap<WeightedIndex<S, usize>>,
}

type WeightedIndex<S, N> = Reverse<(S, N)>;

impl<'a, D, S> Dijkstrable<'a, S, D> for WeightedListGraph<S, D>
where
    S: Clone + Add + Ord + Zero,
    Weighted<S>: Weigh<(usize, S)>,
    D: Direct,
{
    fn dijkstra(&'a self, start: usize) -> Dijkstra<'a, S, D> {
        let mut distance = vec![None; self.len()];
        distance[start] = Some(S::zero());
        let mut p_queue = BinaryHeap::new();
        p_queue.push(Reverse((S::zero(), start)));
        Dijkstra {
            graph: self,
            distance,
            start,
            p_queue,
        }
    }
}
