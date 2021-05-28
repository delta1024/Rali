//! This module houses the wappers and assosiated functions for the user to configure the system and user setting prior to install
use super::{answer_to_bool, ask_for_input, fdisk_output, to_mib};
pub mod drives;
use drives::Drives;
use rpassword::prompt_password_stdout;
pub mod users;
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
fn menu_sections() -> [&'static str; 8] {
    const PROMPT: &str = "Please select an option";
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

    const DRIVE_MENU: &str = "1) Main Drive Id
2) GPT with Bios
3) exit";

    const SWAP_MENU: &str = "1) Swap Partition
2) Swap Size
3) exit";
    const ROOT_MENU: &str = "1) Root File System Size
2) Root File System Format
3) exit";
    const HOME_MENU: &str = "1) Seperate Home Partition
2) Seperate User Home Partition
3) Home Partion Id
4) exit";
    const USER_MENU: &str = "1) User Name
2) Wheel Group
3) Sudoers File
4) Passwords
5) exit";
    const PASSWORDS: &str = "1) User Account
2) Root Account
3) exit";

    [
        PROMPT, SECTIONS, DRIVE_MENU, SWAP_MENU, ROOT_MENU, HOME_MENU, USER_MENU, PASSWORDS,
    ]
}

impl UserSellection {
    pub fn edit(&mut self) -> &mut Self {
        let sections = menu_sections();
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
                            "1" => self.drives.drive_questions(),
                            "2" => self.drives.drive_gpt(),
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
                            "1" => self.drives.swap_part_question(),
                            "2" => self.drives.swap_size_set(),
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
                            "1" => self.drives.root_sys_questions_size(),
                            "2" => self.drives.root_sys_question_format(),
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
                            "1" => self.drives.home_questions_sep_part(),
                            "2" => self.drives.home_questions_have_another_home_part(),
                            "3" => self.drives.home_part_custom_set(),
                            "4" => break,
                            _ => continue,
                        };
                    }
                    continue;
                }
                // user menu
                "5" => {
                    loop {
                        println!("{}", sections[6]);
                        let check = ask_for_input(sections[0]);
                        match check.as_str() {
                            "1" => self.users.name_question(),
                            "2" => self.users.wheel_question(),
                            "3" => self.users.sudoer_question(),
                            "4" => {
                                loop {
                                    println!("{}", sections[7]);
                                    let check = ask_for_input(sections[0]);
                                    match check.as_str() {
                                        "1" => {
                                            self.users.pass_question();
                                            continue;
                                        }
                                        "2" => {
                                            self.set_root_pass();
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

                "6" => break self,
                _ => continue,
            }
        }
    }
}
