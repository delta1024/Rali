//! Holds options related to user configuration
#[allow(unused_imports)]
use super::{answer_to_bool, ask_for_input};
#[derive(Default, Clone)]
pub struct Users {
    user_name: String,
    user_pass: String,
    is_sudoer: bool,
    is_wheel: bool,
    root_pass: String,
}
