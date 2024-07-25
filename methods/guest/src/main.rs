use risc0_zkvm::guest::env;
use ml::Layer;

fn main() {
    // TODO: Implement your guest code here

    // So, we have a single layer in here, which is fed inputs from the previous layer
    // that is on the host side.
    let layer = env::read::<Layer>();
    let input = env::read::<Vec<f32>>();

    let output = layer.forward(&input);

    println!("Output: {:?}", output);
    env::commit(&output);
}
