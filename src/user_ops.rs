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
//! This module houses the wappers and assosiated functions for the user to configure the system and user setting prior to install
use crate::{answer_to_bool, ask_for_input};
pub(crate) mod drives;
use drives::Drives;
use rpassword::prompt_password_stdout;
pub(crate) mod users;
use users::Users;

/// Defines the varrious disk formating options
#[derive(Clone, Debug)]
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
    /// holds user drive config
    pub drives: Drives,
    /// holds user config
    pub users: Users,
    /// holds root user config
    pub root: Users,
}
impl UserSellection {
    pub fn set_root_pass(&mut self) -> &mut Self {
        let root_pass = loop {
            let first_go = prompt_password_stdout("Please enter desired root password:").unwrap();
            let second_go =
                prompt_password_stdout("Please reenter desired root password:").unwrap();

            if first_go == second_go {
                break second_go;
            } else {
                println!("passwords do not match, please try again");
            }
        };
        self.root = Users {
            user_pass: root_pass,
            ..Users::default()
        };
        self
    }
}

impl UserSellection {
    pub(crate) fn edit(&mut self) -> &mut Self {
	crate::menus::user_ops::print_menu(self)
    }
}
