/// Patron struct
/// name: String
/// tab: Vec<(String, f64)>
///    (item, price)
#[derive(Clone, Debug)]
pub struct Patron {
    pub name: String,
    pub tab: Vec<(String, f64)>,
}
