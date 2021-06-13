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
//! setup menu
use crate::{ask_for_input, UserSellection};
/// prompt
const PROMPT: &str = "Please select an option";

/// Option 1 in basic setup review menu
const DRIVE_MENU: [&str; 3] = ["Main Drive Id", "GPT with Bios", "exit"];

/// Option 2 in basic setup review menu
const SWAP_MENU: [&str; 3] = ["Swap Partition", "Swap Size", "exit"];
/// Option 3 in basic setup review menu
const ROOT_MENU: [&str; 3] = ["Root File System Size", "Root File System Format", "exit"];
/// Option 4 in basic setup review menu
const HOME_MENU: [&str; 4] = [
    "Seperate Home Partition",
    "Seperate User Home Partition",
    "Home Partion Id",
    "exit",
];
/// Option 5 in basic setup review menu
const USER_MENU: [&str; 5] = [
    "User Name",
    "Wheel Group",
    "Sudoers File",
    "Passwords",
    "exit",
];
/// sub-options for Option 5
const PASSWORDS: [&str; 3] = ["User Account", "Root Account", "exit"];
/// basic setup review menu
const SECTIONS: [&str; 7] = [
    "
Main Drive Id
GPT with Bios",
    "
Swap Partition
Swap Size
Swap Id",
    "
Root File System 
Root File System Size
Root File System Format",
    "
Seperate Home Partition
Seperate User Home Partition
Home Partion Id",
    "
User Name
Wheel Group
Sudoers File
Passwords",
    "Timezones",
    "exit",
];
pub(crate) fn print_menu(select: &mut UserSellection) -> &mut UserSellection {
    let  main_menu = num_list(SECTIONS.to_vec());
    loop {
        for i in &main_menu {
            println!("{}", i);
        }
        let check = ask_for_input(PROMPT).parse::<usize>().unwrap();
        match check {
            // drive menu
            1 => {
                print_sub_list_drive_menu(select);
                continue;
            }
            // swap menu
            2 => {
                print_sub_list_swap_menu(select);
                continue;
            }
            // root menu
            3 => {
                print_sub_list_root_menu(select);
                continue;
            }
            // home menu
            4 => {
                print_sub_list_home_menu(select);
                continue;
            }
            5 => {
                print_sub_list_user_menu(select);
                continue;
            }

            6 => {
		select.sys.get_time_zone().unwrap();
		continue;
	    }
	    7 => break select,
            _ => continue,
        }
    }
}
/// numbers list
fn num_list(items: Vec<&str>) -> Vec<String> {
    let mut num = 0;
    let items = items
        .iter()
        .map(|s| {
            num += 1;
            format!("{}) {}", num, s)
        })
        .collect();
    items
}
fn print_sub_list_drive_menu(user: &mut UserSellection) {
    let menu = num_list(DRIVE_MENU.to_vec());
    loop {
        for i in &menu {
            println!("{}", i);
        }
        let check = ask_for_input(PROMPT).parse::<usize>().unwrap();
        match check {
            1 => user.drives.drive_questions(),
            2 => user.drives.drive_gpt(),
            3 => break,
            _ => continue,
        };
    }
}
fn print_sub_list_swap_menu(user: &mut UserSellection) {
    let menu = num_list(SWAP_MENU.to_vec());
    loop {
        for i in &menu {
            println!("{}", i);
        }
        let check = ask_for_input(PROMPT);
        match check.as_str() {
            "1" => user.drives.swap_part_question(),
            "2" => user.drives.swap_size_set(),
            "3" => break,
            _ => continue,
        };
    }
}

fn print_sub_list_root_menu(user: &mut UserSellection) {
    let menu = num_list(ROOT_MENU.to_vec());
    loop {
        for i in &menu {
            println!("{}", i);
        }
        let check = ask_for_input(PROMPT).parse::<usize>().unwrap();
        match check {
            1 => user.drives.root_sys_questions_size(),
            2 => user.drives.root_sys_question_format(),
            3 => break,
            _ => continue,
        };
    }
}
fn print_sub_list_home_menu(user: &mut UserSellection) {
    let menu = num_list(HOME_MENU.to_vec());
    loop {
        for i in &menu {
            println!("{}", i);
        }
        let check = ask_for_input(PROMPT).parse::<usize>().unwrap();
        match check {
            1 => user.drives.home_questions_sep_part(),
            2 => user.drives.home_questions_have_another_home_part(),
            3 => user.drives.home_part_custom_set(),
            4 => break,
            _ => continue,
        };
    }
}
fn print_sub_list_user_menu(user: &mut UserSellection) {
    let menu = num_list(USER_MENU.to_vec());
    loop {
	for i in &menu {
	    println!("{}", i);
	}
        let check = ask_for_input(PROMPT).parse::<usize>().unwrap();
        match check{
            1 => user.users.name_question(),
            2 => user.users.wheel_question(),
            3 => user.users.sudoer_question(),
            4 => {
                print_sub_list_passmenu(user);
                continue;
            }
            5 => break,
            _ => continue,
        };
    }
}
fn print_sub_list_passmenu(user: &mut UserSellection) {
    let menu = num_list(PASSWORDS.to_vec());
    loop {
        for i in &menu {
            println!("{}", i);
        }
        let check = ask_for_input(PROMPT).parse::<usize>().unwrap();
        match check {
            1 => {
                user.users.pass_question();
                continue;
            }
            2 => {
                user.set_root_pass();
                continue;
            }
            3 => break,
            _ => continue,
        }
    }
}
