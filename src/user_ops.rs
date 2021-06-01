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
#[allow(dead_code)]
use crate::{answer_to_bool, ask_for_input};
pub(crate) mod drives;
use curl::easy::Easy;
use drives::Drives;
use rpassword::prompt_password_stdout;
pub(crate) mod users;
use self::MirrorOptions::*;
use regex::Regex;
use users::Users;
/// Houses options for fetching user personalised mirrorlist
#[derive(Clone)]
pub(crate) enum MirrorOptions {
    Country(String), // ?country={country}
    Http(String),    // &proticol=http
    Https(String),   // &proticol=https
    IPv4(String),    // &ip_version=4
    IPv6(String),    // &ip_version=6
}
/// Defines the varrious disk formating options
#[derive(Clone)]
pub enum FileSysType {
    Ext4,
    Ext3,
    Btrfs,
    Swap,
}
impl ToString for FileSysType {
    fn to_string(&self) -> String{
	match self {
	    &FileSysType::Ext4 => String::from("ext4"),
	    &FileSysType::Ext3 => String::from("ext3"),
	    &FileSysType::Btrfs => String::from("btrfs"),
	    &FileSysType::Swap => String::from("Swap"),
	}
	
    }
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
#[derive(Default, Clone)]
pub(crate) struct UserSellection {
    /// holds mirror information
    pub(crate) mirrors: Vec<MirrorOptions>,
    /// holds user drive config
    pub(crate) drives: Drives,
    /// holds user config
    pub(crate) users: Users,
    /// holds root user config
    pub(crate) root: Users,
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

    pub(crate) fn query_mirrors(&mut self) -> &mut Self {
        crate::menus::mirrors::print_menu(self)
    }

    pub(crate) fn make_mirror_list(self) -> String {
        let mut url = String::from("https://archlinux.org/mirrorlist/");
        for i in self.mirrors {
            match i {
                Country(c) => url.push_str(&c),
                Http(a) => url.push_str(&a),
                Https(a) => url.push_str(&a),
                IPv4(a) => url.push_str(&a),
                IPv6(a) => url.push_str(&a),
            }
        }
        let mut data = Vec::new();
        let mut handle = Easy::new();
        handle.url(&url).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }
        let mirrorlist = String::from_utf8(data).unwrap();
        let re = Regex::new(r"(?m)^#").unwrap();
	let mirrorlist = re.replace_all(&mirrorlist, "");
	
        mirrorlist.to_string()
    }
}

impl UserSellection {
    #[allow(dead_code)]
    pub(crate) fn edit(&mut self) -> &mut Self {
        crate::menus::user_ops::print_menu(self)
    }
}
