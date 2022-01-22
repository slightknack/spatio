mod quad;
mod ctx;
mod step;
mod render;

fn main() {
    println!("Warming up...");

    let mut ctx = ctx::Ctx::new((), ctx::Basic);

    let mut base = vec![];
    for i in 0..16 {
        base.push(i);
    }

    let mut quad: quad::Node<u8, u8> = quad::Node::new_from_square(
        &mut ctx, base,
    );

    println!("{:#?}", quad);

    // TODO: this does not work correctly
    // for x in -2..2 {
    //     for y in -2..2 {
    //         let (nquad, color) = quad.sample_color(&mut ctx, x, y);
    //         quad = nquad;
    //         println!("{}", color[0]);
    //     }
    // }

    // render::graphics();
}
