use crate::patron::Patron;
use std::sync::Mutex;

// This is a global state of a vector of patrons. 
lazy_static::lazy_static! {
    pub static ref PATRONS: Mutex<Vec<Patron>> = Mutex::new(Vec::new());
}
