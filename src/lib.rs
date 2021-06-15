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
//! RALI aimes to make the installation and redeployment of an arch based system as painless as possible.
//! # TODO
//! * function to create user
//! * imlement sudo configuratoin
//! ** wheel group
//! ** no wheel group
//! * add option for user to have different home partition format
//! * make dynamic menu to only show relevent items
//! * refactor menu for user sellect to be more moduler
//! * create const for basic pacman.conf
//! * Implement toml support

use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
pub(crate) mod user_ops;
pub(crate) use crate::user_ops::UserSellection;
pub(crate) mod menus;
pub(crate) mod packages;
pub(crate) mod parted;

pub fn run() {
    let install_list = packages::BASIC_INSTALL_BIOS;
    let mut choices = user_survay();
    choices.drives.build_drive_ids();
    loop {
        let read_out = format!(
            "\nMain Drive Id: {}
GPT with BIOS: {}
GPT Boot Partition: {}
Swap Partition: {}
Swap Size: {}Mib
Swap Id: {}
Root Filesystem: {}
Root Filesystem Size: {}Mib
Root Filesystem Format: {}
Seperate Home Partition: {}
Custom User Home Partition: {}
Home Partition Id: {}
User Name: {}
Wheel Group: {}
Sudoers File: {}
Timezone: {}",
            choices.drives.drive_id,
            choices.drives.gpt_with_bios,
            choices.drives.gpt_boot_part,
            choices.drives.format_swap,
            choices.drives.swap_size,
            choices.drives.swap_id,
            choices.drives.root_sys_id,
            choices.drives.root_sys_size,
            choices.drives.root_sys_format.to_string(),
            choices.drives.home_part,
            choices.drives.home_part_exist,
            choices.drives.home_id,
            choices.users.user_name,
            choices.users.is_wheel,
            choices.users.is_sudoer,
            choices.sys.time_zone,
        );

        println!("{}", read_out);

        let need_redo = answer_to_bool(ask_for_input("Is this correct? (y/n)"));
        if !need_redo {
            choices.edit();
            choices.drives.build_drive_ids();
        } else {
            break;
        }
    }

    let mirrorlist = choices.clone();
    println!("Downloading mirrorlist");
    let mirrorlist = mirrorlist.make_mirror_list();
    std::fs::write("/etc/pacman.d/mirrorlist", mirrorlist).unwrap();
    println!("Partitioning Drives");
    crate::parted::format(choices.drives.clone()).unwrap();
    let mount = Command::new("/usr/bin/mount")
        .args(&[&choices.drives.root_sys_id, "/mnt"])
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&mount.stdout).unwrap();
    io::stderr().write_all(&mount.stderr).unwrap();

    if choices.drives.format_swap {
        let swap_on = Command::new("/usr/bin/swapon")
            .arg(&choices.drives.swap_id)
            .output()
            .expect("Failed to execute process");
        io::stdout().write_all(&swap_on.stdout).unwrap();
        io::stderr().write_all(&swap_on.stderr).unwrap();
    }
    let install_list: Vec<String> = install_list
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();
    let mut pacstrap = Command::new("/usr/bin/pacstrap")
        .args(install_list)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute process");
    // look in to child process struct
    let mut child_out = BufReader::new(pacstrap.stdout.as_mut().unwrap());
    let mut line = String::new();
    loop {
        line.clear();
        child_out.read_line(&mut line).unwrap();
        println!("{}", line);
        if line == "".to_string() {
            break;
        }
    }
    println!("Generating fstab");
    user_ops::sysops::gen_fstab().unwrap();
    choices.sys.set_net_conf().unwrap();
    choices.sys.set_local().unwrap();
    choices.sys.set_timezone().unwrap();
    choices.sys.set_net_conf().unwrap();
    choices.sys.set_local().unwrap();
    for i in choices.sys.localization {
        println! {"{}", i}
    }
}
#[allow(dead_code)]
fn user_survay() -> UserSellection {
    //! survays the user for their desired system configuration prior to starting the installation process.
    let mut answers = UserSellection::default();

    answers.query_mirrors();
    answers
        .drives
        .drive_questions()
        .drive_gpt()
        .swap_part_question()
        .swap_size_set()
        .root_sys_questions_size()
        .root_sys_question_format()
        .home_questions_sep_part()
        .home_questions_have_another_home_part()
        .home_part_custom_set()
        .home_no_custom_set()
        .build_drive_ids();
    answers
        .users
        .name_question()
        .wheel_question()
        .sudoer_question()
        .pass_question();
    answers.set_root_pass();
    answers
        .sys
        .get_time_zone()
        .unwrap()
        .get_net_conf()
        .get_local();
    answers
}

pub(crate) fn ask_for_input(message: &str) -> String {
    //! Ask the user for confirmation and returns the result
    println!("{}", message);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.pop();
    response
}

pub(crate) fn answer_to_bool(answer: String) -> bool {
    //! converts answer string to bool
    if answer == "y" || answer == "yes" {
        return true;
    } else {
        return false;
    }
}

pub(crate) fn fdisk_output() {
    //! Outputs fdisk -l
    let fdisk_out = Command::new("/usr/bin/fdisk")
        .arg(r#"-l"#)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&fdisk_out.stdout).unwrap();
    io::stderr().write_all(&fdisk_out.stderr).unwrap();
}

pub(crate) fn to_mib(x: String) -> u32 {
    //! converts the given String to the appropriate size value
    let mut x_clone = x.clone();
    let sufix_value = if x.len() == 0 { return 0 } else { x.len() - 1 };
    let disk_size: String = x_clone.drain(..sufix_value).collect();
    let x = disk_size.parse::<u32>().unwrap();
    let n = match x_clone.as_str() {
        "T" => (x * 1000) * 1000,
        "G" => x * 1000,
        "M" => x,
        "k" => x / 1000,
        "b" => (x / 1000) / 1000,
        _ => 0,
    };
    n
}
