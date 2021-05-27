//! holds options related to drive formating
use super::{answer_to_bool, ask_for_input, fdisk_output, to_mib, FileSysType};
#[derive(Default, Clone)]
pub struct Drives {
    pub drive_id: String,
    pub gpt_with_bios: bool,
    pub format_swap: bool,
    pub swap_size: u32,
    pub swap_id: String,
    pub root_id: String,
    pub root_size: u32,
    pub root_format: FileSysType,
    pub home_part: bool,
    pub home_part_exist: bool,
    pub home_part_size: u32,
    pub home_id: String,
}

impl Drives {
    /// asks the user about there drive preferences
    pub fn drive_questions(&mut self) {
        fdisk_output();
        self.drive_id = ask_for_input("Please enter desired drive for partitioning".to_string());

        self.gpt_with_bios = answer_to_bool(ask_for_input(
            "Would you like a gpt partition table".to_string(),
        ));
    }

    /// swap preferences
    pub fn swap_questions(&mut self) {
        self.format_swap = answer_to_bool(ask_for_input(
            "would you like a swap partitoin? (y/n)".to_string(),
        ));
        if self.format_swap {
            self.swap_size = to_mib(ask_for_input(
                "what size would you like your swap partition\ne.g. 5G".to_string(),
            ));
            self.swap_id = {
                let mut drive = self.drive_id.clone();
                drive.push('1');
                drive
            };
        }
    }

    /// root preferences
    pub fn root_questions(&mut self) {
        self.root_id = {
            let mut drive = self.drive_id.clone();
            if self.format_swap {
                drive.push('2');
            } else {
                drive.push('1');
            }
            drive
        };

        self.root_size = {
            to_mib(ask_for_input(
                "What size would you like your root partition to be?\nExample: 20G".to_string(),
            ))
        };

        self.root_format = FileSysType::new(ask_for_input(
            "What format would you like your root drive:\n1) ext4\n2) ext3\n3) btrfs".to_string(),
        ));
    }

    /// home preferences
    pub fn home_questions(&mut self) {
        self.home_part = answer_to_bool(ask_for_input(
            "Would you like a seperate home partition".to_string(),
        ));

        if self.home_part {
            self.home_part_exist = answer_to_bool(ask_for_input(
                "do you already have a sepperate home partition? (y/n)".to_string(),
            ));
            if self.home_part_exist {
                fdisk_output();
                self.home_id = ask_for_input("what is the id of the home partition?".to_string());
            } else {
                self.home_part_size = to_mib(ask_for_input(
                    "What size would you like your home partition to be".to_string(),
                ));
                self.home_id = {
                    let mut drive = self.drive_id.clone();
                    if self.format_swap {
                        drive.push('3');
                    } else {
                        drive.push('2');
                    }
                    drive
                };
            }
        }
    }
}
