use crate::patron::Patron;
use std::sync::Mutex;

lazy_static::lazy_static! {
    /// Vector of `Patron` structs.
    /// This is a global variable that can be accessed from anywhere in the application.
    /// It is wrapped in a `Mutex` to allow for concurrent access.
    /// This is a lazy static variable, meaning it is initialized when it is first used.
    pub static ref PATRONS: Mutex<Vec<Patron>> = Mutex::new(Vec::new());
}
