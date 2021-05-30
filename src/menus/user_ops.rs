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
const DRIVE_MENU: &str = "1) Main Drive Id
2) GPT with Bios
3) exit";

/// Option 2 in basic setup review menu
const SWAP_MENU: &str = "1) Swap Partition
2) Swap Size
3) exit";
/// Option 3 in basic setup review menu
const ROOT_MENU: &str = "1) Root File System Size
2) Root File System Format
3) exit";
/// Option 4 in basic setup review menu
const HOME_MENU: &str = "1) Seperate Home Partition
2) Seperate User Home Partition
3) Home Partion Id
4) exit";
/// Option 5 in basic setup review menu
const USER_MENU: &str = "1) User Name
2) Wheel Group
3) Sudoers File
4) Passwords
5) exit";
/// sub-options for Option 5
const PASSWORDS: &str = "1) User Account
2) Root Account
3) exit";
/// basic setup review menu
const SECTIONS: &str = "1)
Main Drive Id
GPT with Bios
2)
Swap Partition
Swap Size
Swap Id
3)
Root File System 
Root File System Size
Root File System Format
4)
Seperate Home Partition
Seperate User Home Partition
Home Partion Id
5)
User Name
Wheel Group
Sudoers File
Passwords
6) exit";
pub(crate) fn print_menu(select: &mut UserSellection) -> &mut UserSellection {
    let sections = [
        PROMPT, SECTIONS, DRIVE_MENU, SWAP_MENU, ROOT_MENU, HOME_MENU, USER_MENU, PASSWORDS,
    ];
    loop {
        println!("{}", sections[1]);
        let check = ask_for_input(sections[0]);
        match check.as_str() {
            // drive menu
            "1" => {
                loop {
                    println!("{}", sections[2]);
                    let check = ask_for_input(sections[0]);
                    match check.as_str() {
                        "1" => select.drives.drive_questions(),
                        "2" => select.drives.drive_gpt(),
                        "3" => break,
                        _ => continue,
                    };
                }
                continue;
            }
            // swap menu
            "2" => {
                loop {
                    println!("{}", sections[3]);
                    let check = ask_for_input(sections[0]);
                    match check.as_str() {
                        "1" => select.drives.swap_part_question(),
                        "2" => select.drives.swap_size_set(),
                        "3" => break,
                        _ => continue,
                    };
                }
                continue;
            }
            // root menu
            "3" => {
                loop {
                    println!("{}", sections[4]);
                    let check = ask_for_input(sections[0]);
                    match check.as_str() {
                        "1" => select.drives.root_sys_questions_size(),
                        "2" => select.drives.root_sys_question_format(),
                        "3" => break,
                        _ => continue,
                    };
                }
                continue;
            }
            // home menu
            "4" => {
                loop {
                    println!("{}", sections[0]);
                    println!("{}", sections[5]);
                    let check = ask_for_input("");
                    match check.as_str() {
                        "1" => select.drives.home_questions_sep_part(),
                        "2" => select.drives.home_questions_have_another_home_part(),
                        "3" => select.drives.home_part_custom_set(),
                        "4" => break,
                        _ => continue,
                    };
                }
                continue;
            }
            "5" => {
                loop {
                    println!("{}", sections[6]);
                    let check = ask_for_input(sections[0]);
                    match check.as_str() {
                        "1" => select.users.name_question(),
                        "2" => select.users.wheel_question(),
                        "3" => select.users.sudoer_question(),
                        "4" => {
                            loop {
                                println!("{}", sections[7]);
                                let check = ask_for_input(sections[0]);
                                match check.as_str() {
                                    "1" => {
                                        select.users.pass_question();
                                        continue;
                                    }
                                    "2" => {
                                        select.set_root_pass();
                                        continue;
                                    }
                                    "3" => break,
                                    _ => continue,
                                }
                            }
                            continue;
                        }
                        "5" => break,
                        _ => continue,
                    };
                }
                continue;
            }

            "6" => break select,
            _ => continue,
        }
    }
}
