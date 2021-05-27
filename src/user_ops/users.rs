//! Holds options related to user configuration
use super::{answer_to_bool, ask_for_input};
use rpassword::prompt_password_stdout;
#[derive(Default, Clone)]
pub struct Users {
    /// holds user name
    pub user_name: String,
    /// holds the user pass
    pub user_pass: String,
    /// whether or not the user is in the wheel group
    pub is_wheel: bool,
    /// whether or not the users name is in the sudoers file
    pub is_sudoer: bool,
}

impl Users {
    /// prompts the user for there desired user name
    pub fn name_question(&mut self) {
        self.user_name = ask_for_input("What would you like your user name to be?")
    }

    /// sets the users password value
    pub fn pass_question(&mut self) {
        self.user_pass = loop {
            let first_go = prompt_password_stdout("Pleas enter a user password:").unwrap();

            let second_go = prompt_password_stdout("Pleas enter a user password:").unwrap();

            if first_go == second_go {
                break second_go;
            } else {
                println!("passwords do not match, please try again");
            }
        }
    }

    /// sets wheel value
    pub fn wheel_question(&mut self) {
        self.is_wheel = answer_to_bool(ask_for_input("is your user part of the wheel group? (y/n)"))
    }

    /// sets sudoer value
    pub fn sudoer_question(&mut self) {
        if !self.is_wheel {
            self.is_sudoer = answer_to_bool(ask_for_input(
                "Should your user be in the sudoers file instead? (y/n)",
            ))
        }
    }
}
