pub trait Application {
    fn new() -> Self;

    fn run(&self) {
        warn!("Hazel");
    }
}