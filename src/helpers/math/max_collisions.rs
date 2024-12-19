// use factorial::Factorial;
pub fn max_collisions(num_entities: u128) -> usize {
    // factorial method is precise, but not viable for large simulations
    // return ((num_entities + 2 - 1).factorial() / (2 * (num_entities - 1).factorial())) as usize;
    return num_entities as usize * (num_entities as usize - 1) / 2;
}
