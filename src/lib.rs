//! RALI aimes to make the installation and redeployment of an arch based system as painless as possible.
//! # Bugs
//! * Currently the application has no way of knowing what partitions should be mapted to what phisical disk
//!     * need to make it take the argument of /dev/sda and add approriate partition numbers

use rpassword::prompt_password_stdout;
use std::io::{self, Write};
use std::process::Command;
pub mod user_ops;
pub use crate::user_ops::{UserSellection, users::Users};

/// Ask the user for confirmation and returns the result
pub fn ask_for_input(message: &str) -> String {
    println!("{}", message);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.pop();
    response
}

fn fdisk_output() {
    let fdisk_out = Command::new("/usr/bin/fdisk")
        .arg(r#"-l"#)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&fdisk_out.stdout).unwrap();
    io::stderr().write_all(&fdisk_out.stderr).unwrap();
}

/// converts the given String to the appropriate size value
pub fn to_mib(x: String) -> u32 {
    let mut x_clone = x.clone();
    let sufix_value = x.len() - 1;
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

/// converts answer string to bool
pub fn answer_to_bool(answer: String) -> bool {
    if answer == "y" || answer == "yes" {
        return true;
    } else {
        return false;
    }
}

/// survays the user for their desired system configuration prior to starting the installation process.
pub fn user_survay() -> UserSellection {
    let mut answers = UserSellection::default();

    answers.drives.drive_questions();
    answers.drives.drive_gpt();
    answers.drives.swap_part_question();
    answers.drives.swap_size_set();
    answers.drives.root_sys_questions_size();
    answers.drives.root_sys_question_format();
    answers.drives.home_questions_sep_part();
    answers.drives.home_part_custom_set();
    answers.drives.home_no_custom_set();
    answers.users.name_question();
    answers.users.wheel_question();
    answers.users.sudoer_question();
    answers.users.pass_question();

    let root_pass = loop {
        let first_go = prompt_password_stdout("Please enter desired root password:").unwrap();
        let second_go = prompt_password_stdout("Please reenter desired root password:").unwrap();

        if first_go == second_go {
            break second_go;
        } else {
            println!("passwords do not match, please try again");
        }
    };
    answers.root = Users {
        user_pass: root_pass,
        ..Users::default()
    };
    answers
}

pub fn run() {
    let mut choices = user_survay();
    loop {

    let read_out = format!(
        "Main Drive Id: {}
GPT with bois: {}
Swap Partition: {}
Swap Size: {}Mib
Swap Id: {}
Root File System: {}
Root fs Size: {}Mib
Root fs Format: {:?}
Seperate Home Partition: {}
Seperate User Home Part: {}
Home Partition Id: {}
User Name: {}
Wheel Group: {}
Sudoers File: {}",
        choices.drives.drive_id,
        choices.drives.gpt_with_bios,
        choices.drives.format_swap,
        choices.drives.swap_size,
        choices.drives.swap_id,
        choices.drives.root_sys_id,
        choices.drives.root_sys_size,
        choices.drives.root_sys_format,
        choices.drives.home_part,
        choices.drives.home_part_exist,
        choices.drives.home_id,
        choices.users.user_name,
        choices.users.is_wheel,
        choices.users.is_sudoer
    );

    println!("{}", read_out);

    let need_redo = answer_to_bool(ask_for_input("Is this correct? (y/n)"));
    if !need_redo {
        choices.edit();
    }else {
	break

    }

    }
}
