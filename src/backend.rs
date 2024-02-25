pub trait Backend {
    fn run(&mut self, cycles: usize);
}