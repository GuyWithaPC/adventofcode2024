pub trait TruthChecks {
    fn any_true(&mut self) -> bool;
    fn all_true(&mut self) -> bool;
}

impl<T: Iterator<Item = bool>> TruthChecks for T {
    fn any_true(&mut self) -> bool {
        self.any(|b| b)
    }

    fn all_true(&mut self) -> bool {
        self.all(|b| b)
    }
}
