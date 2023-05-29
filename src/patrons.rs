use crate::patron::Patron;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref PATRONS: Mutex<Vec<Patron>> = Mutex::new(Vec::new());
}