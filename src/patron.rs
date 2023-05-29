use crate::patrons::PATRONS;

#[derive(Clone, Debug)]
pub struct Patron {
    pub name: String,
    pub tab: Vec<(String, f64)>,
}

pub fn get_patrons() -> std::sync::MutexGuard<'static, Vec<Patron>> {
    PATRONS.lock().unwrap()
}
