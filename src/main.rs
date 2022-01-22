mod quad;
mod ctx;
mod step;
mod render;

fn main() {
    println!("Warming up...");

    let mut ctx = ctx::Ctx::new_empty();
    let quad: quad::Node<u8, u8> = quad::Node::new_from_square(
        &mut ctx, vec![0xFF;64],
    );

    // render::graphics();

    println!("{:#?}", quad);
}
