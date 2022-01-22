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

    let quad: quad::Node<u8, u8> = quad::Node::new_from_square(
        &mut ctx, base,
    );

    println!("{:#?}", quad);

    // render::graphics();
}
