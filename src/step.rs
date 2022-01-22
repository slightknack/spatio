use crate::ctx::{Ctx, Approx};
use crate::quad::{Node, Embed};

pub fn step<A: Embed, B: Embed, S, N: Approx<A, B, S>>(
    ctx: &mut Ctx<A, B, S, N>,
    node: Node<A, B>
) -> Node<A, B> {
    // pad the graph if needed.

    // if we're at the base, run the cellular automation rule:
    // collect training pair base -> rule.
    // return the updated cached node.

    // forward predict all children nodes of the root
    // collect children nodes into vector X, X'
    // forward predict compressed representation of root Y, Y'
    // our target is to train Y' to match X'.

    // calculate the difference between X and Y.
    // if the difference is below an acceptable threshold:
    // collect training pair Y -> X' (with error Y').
    // return the updated cached node with compr = X'

    // if the difference is above an acceptable threshold:
    // recurse `step` on each child to produce four vectors
    // (the recursion will stop when the true base is reached or threshold is small enough.)
    // collect all children predictions into Z'
    // collect training pair Y -> Z' (with error Y').
    // return the updated cached node with compr = Z'.

    todo!("Implement a step!")
}
