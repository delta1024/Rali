// <rali - (Rust) Arch Linux Installer>
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
// along with this program.  if not, see <https://www.gnu.org/licenses/>.
use std::io::{self, Write};
use std::process::{self, Command};
mod toml_opps;
use std::path::Path;
fn main() {
    println!("Welcome to Arch Linux!");
    let is_uefi_mode = Path::new("/sys/firmware/efi/efivars").exists();
    if is_uefi_mode {
        println!("EFI mode detected");
    } else {
        println!("Bios mode detected");
    }
    let ntp_set_true = Command::new("/usr/bin/timedatectl")
        .arg(r#"set-ntp"#)
        .arg(r#"true"#)
        .status()
        .expect("failed to execute process");
    assert!(ntp_set_true.success());

    let disk_part_list = Command::new("/usr/bin/fdisk")
        .arg(r#"-l"#)
        .output()
        .expect("Failed to execute process");
    println!("\nfdisk -l output:");
    io::stdout().write_all(&disk_part_list.stdout).unwrap();
    io::stderr().write_all(&disk_part_list.stderr).unwrap();
    println!("\nPlease select your drive:");
    println!("Hint: /dev/{{your drive}}");
    let mut user_drive_choice = String::new();

    io::stdin()
        .read_line(&mut user_drive_choice)
        .expect("Failed to read line");
    // user_drive_choice.pop();
    // user_drive_choice.pop();

    println!(
        "You selected: {:?}\n is this correct?\n(y/n)",
        user_drive_choice
    );

    let mut user_confirm = String::new();
    io::stdin()
        .read_line(&mut user_confirm)
        .expect("Failed to read line");

    let user_confirm: &str = user_confirm.as_ref();

    match user_confirm {
        "yes\n" => {
            println!("starting fdisk");
	    println!("{:?}", user_drive_choice);
        }
        "y\n" => {
            println!("starting fdisk");
	    println!("{:?}", user_drive_choice);
        }
        _ => {
            println!("aborting");
            process::exit(2);
        }
    }

    let _part_drive_action = Command::new("/usr/bin/fdisk")
	.arg(user_drive_choice)
	.output()
	.expect("failed to excecute process");
}
