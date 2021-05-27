//! holds options related to drive formating
use super::{answer_to_bool, ask_for_input, fdisk_output, to_mib, FileSysType};
#[derive(Default, Clone)]
pub struct Drives {
    /// Id of the drive to be formated
    pub drive_id: String,
    /// whether or not a gpt parition table is used
    pub gpt_with_bios: bool,
    /// id of the boot drive
    pub gpt_bios_id: String,
    /// whether or not the drive is formated with a swap partition
    pub format_swap: bool,
    /// swap size
    pub swap_size: u32,
    /// swap drive id
    pub swap_id: String,
    /// the root file system partition id
    pub root_sys_id: String,
    /// the root file system partition size
    pub root_sys_size: u32,
    /// the root file system format
    pub root_sys_format: FileSysType,
    /// whether or not a seperate home partition is used
    pub home_part: bool,
    /// whether or not the user already has a home parition they would like to use
    pub home_part_exist: bool,
    /// if the a home partition is being formated what size to make it
    pub home_part_size: u32,
    /// home partition id
    pub home_id: String,
}

impl Drives {
    /// asks the user about there drive preferences
    pub fn drive_questions(&mut self) {
        fdisk_output();
        self.drive_id = ask_for_input("Please enter desired drive for partitioning");
    }

    /// sets gpt partion variable
    pub fn drive_gpt(&mut self) {
        self.gpt_with_bios =
            answer_to_bool(ask_for_input("Would you like a gpt partition table? (y/n)"));
    }
    /// swap preferences
    pub fn swap_part_question(&mut self) {
        self.format_swap = answer_to_bool(ask_for_input("would you like a swap partitoin? (y/n)"));
    }

    pub fn swap_size_set(&mut self) {
        if self.format_swap {
            self.swap_size = to_mib(ask_for_input(
                "what size would you like your swap partition\ne.g. 5G",
            ));
        }
    }

    /// root preferences
    pub fn root_sys_questions_size(&mut self) {
        self.root_sys_size = {
            to_mib(ask_for_input(
                "What size would you like your root partition to be?\nExample: 20G",
            ))
        };
    }
    /// sets the drive format for your disks
    pub fn root_sys_question_format(&mut self) {
            self.root_sys_format = FileSysType::new(ask_for_input(
                "What format would you like your root drive:\n1) ext4\n2) ext3\n3) btrfs",
            ));
    }

    /// seperate home partition
    pub fn home_questions_sep_part(&mut self) {
        self.home_part = answer_to_bool(ask_for_input(
            "Would you like a seperate home partition? (y/n)",
        ));
    }
    /// sets whether or not user already has a home partition
    pub fn home_questions_have_another_home_part(&mut self) {
        self.home_part_exist = answer_to_bool(ask_for_input(
            "do you already have a sepperate home partition? (y/n)",
        ));
    }
    /// sets the id of the custom home partition
    pub fn home_part_custom_set(&mut self) {
        if self.home_part_exist {
            fdisk_output();
            self.home_id = ask_for_input("what is the id of the home partition?");
        }
    }
    /// if no home partition exists asks for the size
    pub fn home_no_custom_set(&mut self) {
        if self.home_part && !self.home_part_exist {
            self.home_part_size = to_mib(ask_for_input(
                "What size would you like your home partition to be",
            ));
        };
    }
}
