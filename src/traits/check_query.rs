pub trait CheckQueryTrait {
    fn t(&self) -> String;
    fn s(&self) -> f64;
    fn r#fn(&self) -> u64;
    fn i(&self) -> u64;
    fn fp(&self) -> u64;
    fn n(&self) -> u8;
}