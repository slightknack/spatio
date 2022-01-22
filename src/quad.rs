use crate::ctx::{Ctx, Approx};

pub trait Embed: Default + Copy + Sized + std::fmt::Debug {

}

/// Represents the data present in a quad-tree node.
/// May be the base-level repr, or a node with 4 children.
#[derive(Debug)]
pub enum Quad<A: Embed, B: Embed> {
    /// Base cell in grid.
    /// May actually be a chunk of cells for performance.
    Base(A),
    /// Node with 4 children.
    Node(Box<[Node<A, B>;4]>),
    /// Children may be generated from Node.
    Cached,
}

impl Embed for u8 {}

/// Represends a node in a quadtree.
/// Has a depth denoting the number of nodes below it.
/// The base node has a depth of 0
/// Nodes should only be siblings of nodes with the same depth.
/// Data stored inside a quadtree node, including children, are in `data`.
#[derive(Debug)]
pub struct Node<A: Embed, B: Embed> {
    pub depth: usize,
    pub compr: B,
    pub data:  Quad<A, B>,
}

impl<A: Embed, B: Embed> Node<A, B> {
    // TODO: this function does not work correctly
    pub fn sample_color<S, N: Approx<A, B, S>>(
        self,
        ctx: &mut Ctx<A, B, S, N>,
        x: isize,
        y: isize,
    ) -> (Self, [u8; 4]) {
        let mut expanded = ctx.expand(self);
        match expanded.data {
            Quad::Base(a) => (expanded, ctx.color_base(a)),
            Quad::Node(ref mut n) => {
                let depth = expanded.depth;
                let half = 1 << (depth - 1);
                let (c, nx, ny) = match (x >= 0, y >= 0) {
                    (true  ,  true) => (1, x - half, y - half),
                    (true  , false) => (3, x - half, y + half),
                    (false ,  true) => (0, x + half, y - half),
                    (false , false) => (2, x + half, y + half),
                };

                let oc = std::mem::replace(&mut n[c], Self::new_empty(ctx));
                let (nc, color) = oc.sample_color(ctx, nx, ny);
                n[c] = nc;

                (expanded, color)
            },
            Quad::Cached => unreachable!("Expanded this node!"),
        }
    }

    /// Creates a new tree with a single empty base node
    pub fn new_empty<S, N: Approx<A, B, S>>(ctx: &mut Ctx<A, B, S, N>) -> Self {
        Self::new_base(ctx, Default::default())
    }

    /// Creates a new tree from a single base node
    pub fn new_base<S, N: Approx<A, B, S>>(ctx: &mut Ctx<A, B, S, N>, base: A) -> Self {
        Node {
            depth: 0,
            compr: ctx.compress_base(base),
            data:  Quad::Base(base),
        }
    }

    pub fn new_node<S, N: Approx<A, B, S>>(ctx: &mut Ctx<A, B, S, N>, children: [Node<A, B>;4]) -> Self {
        // Make sure the depths check out
        assert_eq!(children[0].depth, children[1].depth);
        assert_eq!(children[1].depth, children[2].depth);
        assert_eq!(children[2].depth, children[3].depth);
        let depth = children[0].depth + 1;

        // compress and build the node
        let quad = Quad::Node(Box::new(children));
        let compr = ctx.compress(&quad).unwrap();
        Node { depth, compr, data: quad }
    }

    pub fn new_cached(compr: B, depth: usize) -> Self {
        Node {
            depth,
            compr,
            data: Quad::Cached,
        }
    }

    fn build_square<S, N: Approx<A, B, S>>(
        ctx: &mut Ctx<A, B, S, N>,
        square: &[A],
        abs_size: usize,
        depth: usize,
        x: isize,
        y: isize,
    ) -> Node<A, B> {
        if depth == 0 {
            let abs_x = (x + (abs_size / 2) as isize) as usize;
            let abs_y = (y + (abs_size / 2) as isize) as usize;
            let idx = abs_y * abs_size + abs_x;
            Self::new_base(ctx, square[idx])
        } else {
            let size = 1 << depth;
            let half = size / 2;
            // in a z-like pattern
            let children = [
                Self::build_square(ctx, square, abs_size, depth - 1, x       , y       ),
                Self::build_square(ctx, square, abs_size, depth - 1, x + half, y       ),
                Self::build_square(ctx, square, abs_size, depth - 1, x       , y + half),
                Self::build_square(ctx, square, abs_size, depth - 1, x + half, y + half),

            ];
            Self::new_node(ctx, children)
        }
    }

    pub fn new_from_square<S, N: Approx<A, B, S>>(ctx: &mut Ctx<A, B, S, N>, square: Vec<A>) -> Self {
        // get the side length, ensure this is a pow2 square
        let area = square.len();
        let size = ((area as f64).sqrt() + 0.5) as usize;
        dbg!(size);
        assert!(size.is_power_of_two());
        assert!(size * size == area);

        // Takes the log2 of the number
        // Power of two, so `size - 1` is all ones
        let depth: usize = (size - 1).trailing_ones() as usize;
        let half = (size / 2) as isize;

        // Start in lower-left corner (half, half),
        // build up and out recursively
        Self::build_square(ctx, &square, size, depth, -half, -half)
    }

    /// Creates a new node double the size by centering the current node
    /// on a node double the size.
    pub fn pad_empty<S, N: Approx<A, B, S>>(self, ctx: &mut Ctx<A, B, S, N>) -> Self {
        todo!()
    }
}
