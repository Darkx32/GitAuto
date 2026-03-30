pub trait BetterString {
    fn clean(&self) -> String;
}

impl BetterString for String {
    fn clean(&self) -> String {
        self.trim().into()
    }
}