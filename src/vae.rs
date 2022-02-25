use tch::nn::{Module, OptimizerConfig};
use tch::{kind, nn, Device, Tensor};

pub fn vae(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(vs, 100, 50, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs, 50, 10, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs, 10, 50, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(vs, 50, 100, Default::default()))
}
