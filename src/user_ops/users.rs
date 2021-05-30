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
//! Holds options related to user configuration
use super::{answer_to_bool, ask_for_input};
use rpassword::prompt_password_stdout;
#[derive(Default, Clone)]
pub struct Users {
    /// holds user name
    pub user_name: String,
    /// whether or not the user is in the wheel group
    pub is_wheel: bool,
    /// whether or not the users name is in the sudoers file
    pub is_sudoer: bool,
    /// holds the user pass
    pub user_pass: String,
}

impl Users {
    /// prompts the user for there desired user name
    pub fn name_question(&mut self) -> &mut Self {
        self.user_name = ask_for_input("What would you like your user name to be?");
        self
    }

    /// sets wheel value
    pub fn wheel_question(&mut self) -> &mut Self {
        self.is_wheel =
            answer_to_bool(ask_for_input("is your user part of the wheel group? (y/n)"));
        self
    }

    /// sets sudoer value
    pub fn sudoer_question(&mut self) -> &mut Self {
        if !self.is_wheel {
            self.is_sudoer = answer_to_bool(ask_for_input(
                "Should your user be in the sudoers file instead? (y/n)",
            ))
        }
        self
    }
    /// sets the users password value
    pub fn pass_question(&mut self) -> &mut Self {
        self.user_pass = loop {
            let first_go = prompt_password_stdout("Pleas enter your user password:").unwrap();

            let second_go = prompt_password_stdout("Pleas reenter your user password:").unwrap();

            if first_go == second_go {
                break second_go;
            } else {
                println!("passwords do not match, please try again");
            }
        };
        self
    }
}
