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
// along with this program.  if not, see <https://www.gnu.org/licenses/>.
mod toml_opps;
use std::io::{self, Write};
use std::path::Path;
use std::process::{self, Command};
fn fdisk_output() {
    let fdisk_out = Command::new("/usr/bin/fdisk")
        .arg(r#"-l"#)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&fdisk_out.stdout).unwrap();
    io::stderr().write_all(&fdisk_out.stderr).unwrap();
}

/// This module houses all of the fucitons related to the formating of Master Boot Record partitions
pub mod mbr_func {
    use mbrman;
    /// This fuction is designed to be used in conjunciton with an already formated disk.
    /// # Panics
    /// * Using it on a unformated drive results in a panic.
    pub fn list_partitions(disk: String) {
        let mut f = std::fs::File::open(disk).expect("could not open disk");
        let mbr = mbrman::MBR::read_from(&mut f, 512).expect("could not find MBR");
        println!("Disk signature: {:?}", mbr.header.disk_signature);

        for (i, p) in mbr.iter() {
            if p.is_used() {
                let byte_as_usize: usize = p.sectors as usize * mbr.sector_size as usize;
                println!(
                    "Partition #{}: type = {:?}, size = {} bytes, starting lba = {}",
                    i, p.sys, byte_as_usize, p.starting_lba
                );
            }
        }
    }
}

/// Ask the user for confirmation and returns the result
fn ask_for_confirm(message: String) -> String {
    println!("{}", message);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.pop();
    response
}

/// converts the given String to the appropriate sector value
fn to_sectors(x: String) -> i64 {
    let mut x = x;
    let sufix_value = x.len() - 1;
    let disk_size: String = x.drain(..sufix_value).collect();
    println!("Disk Size: {}\n Sufix: {}", disk_size, x);
    64
    // match x {
    // 	"G" => ,
    // 	"M" => ,
    // 	"k" => ,
    // 	"b" => ,
    // }
}

fn main() {
    println!("Welcome to Arch Linux!");
    let is_uefi_mode = Path::new("/sys/firmware/efi/efivars").exists();
    if is_uefi_mode {
        println!("EFI mode detected");
    } else {
        println!("BIOS mode detected");
    }
    let correct_mode_confirm = String::from("Is this correct? (y/n)");
    let correct_mode_confirm = ask_for_confirm(correct_mode_confirm);
    let correct_mode_confirm = if correct_mode_confirm == "y" || correct_mode_confirm == "yes" {
        true
    } else {
        false
    };
    if !correct_mode_confirm {
	println!("exiting");
        process::exit(4)
    }
    let ntp_set_true = Command::new("/usr/bin/timedatectl")
        .arg(r#"set-ntp"#)
        .arg(r#"true"#)
        .status()
        .expect("failed to execute process");
    assert!(ntp_set_true.success());

    fdisk_output();
    let user_drive = String::from("Please enter desired drive for partitioning");
    let _user_drive = ask_for_confirm(user_drive);
    // * Ask the user if they wish to create a swap partition and if so what size
    let user_swap = String::from("Do you wish to hav a swap partition? (y/n)");
    let user_swap = ask_for_confirm(user_swap);
    let user_swap = if user_swap == "y" || user_swap == "yes" {
        true
    } else {
        false
    };
    let user_swap_size = if user_swap {
        let swap_size = String::from(
            "What size do you wish to make the swap partition
(G)b (M)b (k)b (b)\n example: 512M",
        );
        let user_swap_size = ask_for_confirm(swap_size);
        to_sectors(user_swap_size)
    } else {
        // set to arbitrary number so we can drop the value if it's not used
        0
    };
    if !user_swap {
        drop(user_swap_size);
    }
}

// fn run() {

// }
