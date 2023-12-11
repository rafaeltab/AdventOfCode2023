pub trait NumberProvider {
    fn extract_numbers(&self, text: &str) -> Vec<Vec<u32>>;
}
