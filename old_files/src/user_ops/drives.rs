// <RALI - Rali, the Arch Linux Installer>
// Copyright (c) <2021>  <Jacob Stannix>
//
// this program is free software: you can redistribute it and/or modify
// it under the terms of the gnu general public license as published by
// the free software foundation, either version 3 of the license, or
// (at your option) any later version.
//
// this program is distributed in the hope that it will be useful,
// but without any warranty; without even the implied warranty of
// merchantability or fitness for a particular purpose.  see the
// gnu general public license for more details.
//
// you should have received a copy of the gnu general public license
// along with this program.  if not, see <https://www.gnu.org/licenses/>
//! holds options related to drive formating
use crate::{answer_to_bool, ask_for_input, fdisk_output, to_mib, user_ops::FileSysType};
#[derive(Default, Clone)]
/// holds all drive options
pub struct Drives {
    /// Id of the drive to be formated
    pub drive_id: String,
    /// boot partition for bios gpt tables
    pub gpt_boot_part: String,
    /// whether or not a gpt parition table is used
    pub gpt_with_bios: bool,
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
    pub fn drive_questions(&mut self) -> &mut Self {
        fdisk_output();
        self.drive_id = ask_for_input("Please enter desired drive for partitioning");
        self
    }

    /// sets gpt partion variable
    pub fn drive_gpt(&mut self) -> &mut Self {
        self.gpt_with_bios =
            answer_to_bool(ask_for_input("Would you like a gpt partition table? (y/n)"));
        self
    }
    /// swap preferences
    pub fn swap_part_question(&mut self) -> &mut Self {
        self.format_swap = answer_to_bool(ask_for_input("would you like a swap partitoin? (y/n)"));
        self
    }

    pub fn swap_size_set(&mut self) -> &mut Self {
        if self.format_swap {
            self.swap_size = to_mib(ask_for_input(
                "what size would you like your swap partition\ne.g. 5G",
            ));
        }
        self
    }

    /// root preferences
    pub fn root_sys_questions_size(&mut self) -> &mut Self {
        self.root_sys_size = {
            to_mib(ask_for_input(
                "What size would you like your root partition to be?\nExample: 20G",
            ))
        };
        self
    }
    /// sets the drive format for your disks
    pub fn root_sys_question_format(&mut self) -> &mut Self {
        self.root_sys_format = FileSysType::new(ask_for_input(
            "What format would you like your root drive:\n1) ext4\n2) ext3\n3) btrfs",
        ));
        self
    }

    /// seperate home partition
    pub fn home_questions_sep_part(&mut self) -> &mut Self {
        self.home_part = answer_to_bool(ask_for_input(
            "Would you like a seperate home partition? (y/n)",
        ));
        self
    }
    /// sets whether or not user already has a home partition
    pub fn home_questions_have_another_home_part(&mut self) -> &mut Self {
        if self.home_part {
            self.home_part_exist = answer_to_bool(ask_for_input(
                "do you already have a sepperate home partition? (y/n)",
            ));
        }
        self
    }
    /// sets the id of the custom home partition
    pub fn home_part_custom_set(&mut self) -> &mut Self {
        if self.home_part_exist {
            fdisk_output();
            self.home_id = ask_for_input("what is the id of the home partition?");
        }
        self
    }
    /// if no home partition exists asks for the size
    pub fn home_no_custom_set(&mut self) -> &mut Self {
        if self.home_part && !self.home_part_exist {
            self.home_part_size = to_mib(ask_for_input(
                "What size would you like your home partition to be\ne.g. 20G",
            ));
        };
        self
    }

    /// sets the drive ids per linux specifications
    pub fn build_drive_ids(&mut self) -> &mut Self {
        let num_user_sellect = vec![self.gpt_with_bios, self.format_swap, self.home_part];
        let mut count = 0;
        let drive_prime = self.drive_id.clone();
        for i in num_user_sellect {
            if i {
                count += 1;
            }
        }
        self.swap_id = String::new();
        self.root_sys_id = String::new();
        if !self.home_part_exist {
            self.root_sys_id = String::new();
        }
        match count {
            0 => {
                let mut drive = drive_prime.clone();
                drive.push('1');
                self.root_sys_id = drive;
                self
            }
            1 => {
                // if one is true you need two partitions
                // check for gpt with bios ro format swapt for which variable to push to
                if self.gpt_with_bios {
                    let mut bios_drive = drive_prime.clone();
                    let mut drive = drive_prime.clone();
                    bios_drive.push('1');
                    drive.push('2');
                    self.root_sys_id = drive;
                    self.gpt_boot_part = bios_drive;
                    self
                } else if self.format_swap {
                    let mut swap_drive = drive_prime.clone();
                    let mut drive = drive_prime.clone();
                    swap_drive.push('1');
                    drive.push('2');
                    self.root_sys_id = drive;
                    self.swap_id = swap_drive;
                    self
                } else {
                    if !self.home_part_exist {
                        let mut home_drive = drive_prime.clone();
                        let mut drive = drive_prime.clone();
                        drive.push('1');
                        home_drive.push('2');
                        self.root_sys_id = drive;
                        self.home_id = home_drive;
                        self
                    } else {
                        let mut drive = drive_prime.clone();
                        drive.push('1');
                        self.root_sys_id = drive;
                        self
                    }
                }
            }
            2 => {
                if self.gpt_with_bios && self.format_swap {
                    let mut drive = drive_prime.clone();
                    let mut bios_drive = drive_prime.clone();
                    let mut swap_drive = drive_prime.clone();
                    drive.push('3');
                    bios_drive.push('1');
                    swap_drive.push('2');
                    self.root_sys_id = drive;
                    self.gpt_boot_part = bios_drive;
                    self.swap_id = swap_drive;
                    self
                } else if self.format_swap && self.home_part {
                    let mut drive = drive_prime.clone();
                    let mut home_drive = drive_prime.clone();
                    let mut swap_drive = drive_prime.clone();
                    drive.push('2');
                    home_drive.push('3');
                    swap_drive.push('1');
                    self.root_sys_id = drive;
                    self.home_id = home_drive;
                    self.swap_id = swap_drive;
                    self
                } else {
                    let mut drive = drive_prime.clone();
                    let mut bios_drive = drive_prime.clone();
                    let mut home_drive = drive_prime.clone();
                    drive.push('2');
                    bios_drive.push('1');
                    home_drive.push('3');
                    self.root_sys_id = drive;
                    self.gpt_boot_part = bios_drive;
                    self.home_id = home_drive;
                    self
                }
            }
            3 => {
                // let num_user_sellect = vec![self.gpt_with_bios, self.format_swap, self.home_part];
                let mut bios_drive = drive_prime.clone();
                let mut home_drive = drive_prime.clone();
                let mut swap_drive = drive_prime.clone();
                let mut drive = drive_prime.clone();

                bios_drive.push('1');
                swap_drive.push('2');
                drive.push('3');
                if !self.home_part_exist {
                    home_drive.push('4');
                    self.home_id = home_drive;
                }
                self.gpt_boot_part = bios_drive;
                self.swap_id = swap_drive;
                self.root_sys_id = drive;
                self
            }
            _ => panic!("help"),
        }
    }
}
