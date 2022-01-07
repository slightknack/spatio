use crate::quad::{Quad, Node};

/// Represents a context with shared state.
pub struct Ctx();

impl Ctx {
    /// Creates a new uninitialized context.
    pub fn new_empty() -> Self {
        Ctx()
    }

    /// Combines 4 child node representations into a single representation
    /// Using a neural network.
    pub fn combine<B>(&mut self, compr: [B; 4]) -> B {
        todo!("Build new B from 4 child B");
    }

    /// Compresses a base-level cell into a vector.
    pub fn compress_base<A, B>(&mut self, base: A) -> B {
        todo!("Turn Base Cell into a vector B");
    }

    /// Compresses a single node into a vector representation.
    /// Returns `None` if node has already been compressed and trimmed from tree.
    /// To recover a trimmed node, use `expand` on the compressed representation.
    pub fn compress<A: Default, B: Copy>(&mut self, quad: &Quad<A, B>) -> Option<B> {
        match quad {
            Quad::Base(b) => Some(self.compress_base(b)),
            Quad::Node(n) => Some(
                self.combine([
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
    fn expand_base<A: Default, B: Copy>(&mut self, compr: B) -> A {
        todo!("Turn compressed B into the A that made it");
    }

    fn expand_node<B>(&mut self, compr: B) -> [B; 4] {
        todo!("Turn compressed B into 4 child B that made it");
    }

    /// Expands the compressed representation of a node into a node with 4 children.
    pub fn expand<A: Default, B: Copy>(&mut self, mut compr: Node<A, B>) -> Node<A, B> {
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
