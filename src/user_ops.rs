//! This module houses the wappers and assosiated functions for interacting with parted through the command line
use super::{answer_to_bool, ask_for_input, fdisk_output, to_mib};
pub mod drives;
use drives::Drives;
pub mod users;
use users::Users;
/// Defines the varrious disk formating options
#[derive(Clone)]
pub enum FileSysType {
    Ext4,
    Ext3,
    Btrfs,
    Swap,
}
impl FileSysType {
    pub fn new(answer: String) -> Self {
        match answer.as_str() {
            "1" => FileSysType::Ext4,
            "2" => FileSysType::Ext3,
            "3" => FileSysType::Btrfs,
            "4" => FileSysType::Swap,
	    _ => FileSysType::Ext4,
        }
    }
}
impl Default for FileSysType {
    fn default() -> Self {
        FileSysType::Ext4
    }
}
/// holds user choices about configuration
#[derive(Default)]
pub struct UserSellection {
    pub drives: Drives,
    pub users: Users,
}
