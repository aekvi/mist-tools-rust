pub trait InitHandler {
    fn execute(&self) -> Result<(), &'static str>;
}
