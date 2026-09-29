
use web_sys;

pub fn print(str: &str) {
    web_sys::console::log_1(
            &str.into()
    );
}
