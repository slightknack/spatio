use crate::ctx::Ctx;

/// Represents the data present in a quad-tree node.
/// May be the base-level repr, or a node with 4 children.
pub enum Quad<A: Default, B: Copy> {
    /// Base cell in grid.
    /// May actually be a chunk of cells for performance.
    Base(A),
    /// Node with 4 children.
    Node(Box<[Node<A, B>;4]>),
    /// Children may be generated from Node.
    Cached,
}

/// Represends a node in a quadtree.
/// Has a depth denoting the number of nodes below it.
/// The base node has a depth of 0
/// Nodes should only be siblings of nodes with the same depth.
/// Data stored inside a quadtree node, including children, are in `data`.
pub struct Node<A: Default, B: Copy> {
    pub depth: usize,
    pub compr: B,
    pub data:  Quad<A, B>,
}

impl<A: Default, B: Copy> Node<A, B> {
    /// Bytes must be of size (2**depth)**2 * 4
    pub fn write_rgba8(
        self,
        ctx: &mut Ctx,
        bytes: &mut [u8],
        depth: usize,
        x: usize,
        y: usize,
    ) -> Self {
        let mut expanded = ctx.expand(self);
        expanded.data = match expanded.data {
            // Base cell in grid.
            Quad::Base(base) => {
                let mut color = ctx.color_base(&base);
                let idx = ((1 << depth) * y + x) * 4;
                bytes[idx..(idx + 4)].swap_with_slice(&mut color);
                Quad::Base(base)
            },
            // Node with 4 children.
            Quad::Node(children) => {
                let size = 1 << expanded.depth;
                let half = size / 2;
                let c = [
                    children[0].write_rgba8(ctx, bytes, depth, x,        y       ),
                    children[1].write_rgba8(ctx, bytes, depth, x + half, y       ),
                    children[2].write_rgba8(ctx, bytes, depth, x,        y + half),
                    children[3].write_rgba8(ctx, bytes, depth, x + half, y + half),
                ];
                todo!()
            },
            // Children may be generated from Node.
            Cached => { unreachable!(); },
        };
        expanded
    }

    /// Creates a new tree from a single base node
    pub fn new_base(base: A, ctx: &mut Ctx) -> Self {
        Node {
            depth: 0,
            compr: ctx.compress_base(&base),
            data:  Quad::Base(base),
        }
    }

    /// Creates a new tree with a single empty base node
    pub fn new_empty(ctx: &mut Ctx) -> Self {
        Self::new_base(Default::default(), ctx)
    }

    pub fn new_cached(compr: B, depth: usize) -> Self {
        Node {
            depth,
            compr,
            data: Quad::Cached,
        }
    }

    /// Creates a new node double the size by centering the current node
    /// on a node double the size.
    pub fn pad_empty(self, ctx: &mut Ctx) -> Self {
        todo!()
    }
}
