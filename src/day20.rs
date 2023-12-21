use std::collections::HashSet;

#[derive(Debug, Clone)]
enum Module {
    BroadCaster,
    FlipFlop(Signal),
    Conjonction(Vec<Signal>),
}

#[derive(Debug, Clone, Copy)]
enum Signal {
    Low,
    High,
}

fn main() {
    todo!()
}
