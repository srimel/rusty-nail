// current_patron.rs

static mut CURRENT_PATRON: Option<String> = None;

pub fn set_current_patron(patron_name: &str) {
    unsafe {
        CURRENT_PATRON = Some(patron_name.to_string());
    }
}

pub fn get_current_patron() -> Option<String> {
    unsafe { CURRENT_PATRON.clone() }
}