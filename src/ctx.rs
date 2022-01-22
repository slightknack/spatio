use std::marker::PhantomData;
use crate::quad::{Quad, Node, Embed};

pub trait Approx<A: Embed, B: Embed, S>: Default {
    fn color_base(state: &mut S, base: A) -> [u8; 4];

    fn compress_node(state: &mut S, compr: [B; 4]) -> B;
    fn   expand_node(state: &mut S, compr: B) -> [B; 4];

    fn compress_base(state: &mut S, embed: A) -> B;
    fn   expand_base(state: &mut S, compr: B) -> A;
}

#[derive(Default)]
pub struct Basic;

impl Approx<u8, u8, ()> for Basic {
    fn color_base(state: &mut (), base: u8) -> [u8; 4] {
        [base;4]
    }

    fn compress_node(state: &mut (), c: [u8; 4]) -> u8 {
        c[0]/4 + c[1]/4 + c[2]/4 + c[3]/4
    }

    fn expand_node(state: &mut (), compr: u8) -> [u8; 4] {
        [compr;4]
    }

    fn compress_base(state: &mut (), embed: u8) -> u8 {
        embed
    }

    fn expand_base(state: &mut (), compr: u8) -> u8 {
        compr
    }
}

/// Represents a context with shared state.
#[derive(Debug)]
pub struct Ctx<A: Embed, B: Embed, S, N: Approx<A, B, S>> {
    _phantom_a: PhantomData<A>,
    _phantom_b: PhantomData<B>,
    state:    S,
    networks: N,
}

impl<A: Embed, B: Embed, S, N: Approx<A, B, S>> Ctx<A, B, S, N> {
    /// Creates a new uninitialized context.
    pub fn new(state: S, networks: N) -> Self {
        Ctx {
            _phantom_a: PhantomData,
            _phantom_b: PhantomData,
            state,
            networks,
        }
    }

    pub fn color_base(&mut self, base: A) -> [u8; 4] {
        N::color_base(&mut self.state, base)
    }

    /// Compresses a base-level cell into a vector.
    pub fn compress_base(&mut self, base: A) -> B {
        N::compress_base(&mut self.state, base)
    }

    /// Combines 4 child node representations into a single representation
    /// Using a neural network.
    pub fn compress_node(&mut self, compr: [B; 4]) -> B {
        N::compress_node(&mut self.state, compr)
    }

    /// Compresses a single node into a vector representation.
    /// Returns `None` if node has already been compressed and trimmed from tree.
    /// To recover a trimmed node, use `expand` on the compressed representation.
    pub fn compress(&mut self, quad: &Quad<A, B>) -> Option<B> {
        match quad {
            Quad::Base(b) => Some(self.compress_base(*b)),
            Quad::Node(n) => Some(
                self.compress_node([
                    n[0].compr,
                    n[1].compr,
                    n[2].compr,
                    n[3].compr,
                ])
            ),
            Quad::Cached => None,
        }
    }

    /// Compresses a base-level cell into a vector.
    pub fn expand_base(&mut self, compr: B) -> A {
        N::expand_base(&mut self.state, compr)
    }

    pub fn expand_node(&mut self, compr: B) -> [B; 4] {
        N::expand_node(&mut self.state, compr)
    }

    /// Expands the compressed representation of a node into a node with 4 children.
    pub fn expand(&mut self, mut compr: Node<A, B>) -> Node<A, B> {
        match compr.data {
            // Can't expand a base node.
            Quad::Base(_) => {
                debug_assert!(compr.depth == 0, "Tree is malformed at the leaves");
                compr
            },

            // No-op if node is already expanded.
            Quad::Node(_) => {
                debug_assert!(compr.depth != 0, "Tree is malformed along the trunk");
                compr
            },

            Quad::Cached => {
                // Expand and repace the current node data.
                compr.data = if compr.depth == 0 {
                    // Base case.
                    Quad::Base(self.expand_base(compr.compr))
                } else {
                    // Expand the children into their corresponding vectors.
                    let c = self.expand_node(compr.compr);
                    // Pack the children into a new Node.
                    let new_depth = compr.depth - 1;
                    let children = [
                        Node::new_cached(c[0], new_depth),
                        Node::new_cached(c[1], new_depth),
                        Node::new_cached(c[2], new_depth),
                        Node::new_cached(c[3], new_depth),
                    ];
                    // Heap-allocate the node lol.
                    Quad::Node(Box::new(children))
                };

                // The updated node.
                compr
            },
        }
    }
}
