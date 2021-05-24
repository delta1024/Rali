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
mod toml_opps;
use mbrman;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
#[allow(dead_code)]
fn fdisk_output() {
    let fdisk_out = Command::new("/usr/bin/fdisk")
	.arg(r#"-l"#)
	.output()
	.expect("Failed to execute process");
    io::stdout().write_all(&fdisk_out.stdout).unwrap();
    io::stderr().write_all(&fdisk_out.stderr).unwrap();
}

fn list_partitions(disk: String) {
    let mut f = std::fs::File::open(disk).expect("could not open disk");
    let mbr = mbrman::MBR::read_from(&mut f, 512).expect("could not find MBR");
    println!("Disk signature: {:?}", mbr.header.disk_signature);

    for (i, p) in mbr.iter() {
        if p.is_used() {
	    let byte_as_usize: usize = p.sectors as usize * mbr.sector_size as usize;
            println!(
                "Partition #{}: type = {:?}, size = {} bytes, starting lba = {}",
                i,
                p.sys,
		byte_as_usize,
                p.starting_lba
            );
        }
    }
}


fn main() {
    println!("Welcome to Arch Linux!");
    let is_uefi_mode = Path::new("/sys/firmware/efi/efivars").exists();
   if is_uefi_mode {
        println!("EFI mode detected");
    } else {
        println!("BIOS mode detected");
    }
    let ntp_set_true = Command::new("/usr/bin/timedatectl")
        .arg(r#"set-ntp"#)
        .arg(r#"true"#)
        .status()
        .expect("failed to execute process");
    assert!(ntp_set_true.success());

    fdisk_output();
    println!("Please enter desired drive for partitioning");
    let mut user_drive = String::new();
    io::stdin()
        .read_line(&mut user_drive)
        .expect("Failed to read line");

    user_drive.pop();
    list_partitions(user_drive);
}
