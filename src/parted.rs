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
//! wrapper for parted specific to the needs of this application
use crate::user_ops::drives::Drives;
use std::io::{self, Write};
fn partition_command(drive: Drives) -> Vec<String> {
    let mut drive_commands: Vec<String> = vec!["--script".to_string()];
    let device = format! {"{}", drive.drive_id};
    let sizes = DriveSize::new(&drive);
    drive_commands.push(device);
    if drive.gpt_with_bios {
        drive_commands.push("mklabel gpt".to_string());
        drive_commands.push("mkpart primary 1Mib 512Mib".to_string());
        drive_commands.push("set 1 \"bios_grub\" on".to_string())
    } else {
        drive_commands.push("mklabel msdos".to_string());
    }
    if drive.format_swap {
        drive_commands.push(format!(
            "mkpart primary linux-swap {}Mib {}Mib",
            sizes.swap_start, sizes.swap_end
        ));
        if drive.gpt_with_bios {
            drive_commands.push("set 2 \"swap\" on".to_string());
        }
    }
    drive_commands.push(format!(
        "mkpart primary {}Mib {}Mib",
        sizes.root_start, sizes.root_end
    ));
    if drive.home_part && !drive.home_part_exist {
        drive_commands.push(format!(
            "mkpart primary {}Mib {}Mib",
            sizes.home_start, sizes.home_end
        ))
    }
    drive_commands
}
#[derive(Default)]
struct DriveSize {
    _bios: u32,
    swap_start: u32,
    swap_end: u32,
    root_start: u32,
    root_end: u32,
    home_start: u32,
    home_end: u32,
}
// parted --script /device \
//     mklabel gpt \
//     mkpart primary 1MiB 512MiB \ bios drive
//     mkpart primary 512MiB {512 + swap_size}MiB \ swap drive
//     mkpart primary {swap_end} {swap_end + root_size}\ root drive
//     mkpart primary {root_end} {root_end + home_size} \ home drive
impl DriveSize {
    fn new(tests: &Drives) -> Self {
        let mut tests = tests.clone();
        let mut sizes = DriveSize::default();
        if tests.gpt_with_bios {
            if tests.format_swap {
                sizes.swap_start = 512;
            } else {
                sizes.root_start = 512;
            }
        } else {
            if tests.format_swap {
                sizes.swap_start = 1;
            } else {
                sizes.root_start = 1;
            }
        }
        if tests.format_swap {
            sizes.swap_end = sizes.swap_start + tests.swap_size;
            sizes.root_start = sizes.swap_end;
        }
        let root_size = if tests.root_sys_size == 0 {
            if tests.home_part && !tests.home_part_exist {
                tests.root_sys_questions_size();
                tests.root_sys_size
            } else {
                rest_of_disk(sizes.root_start.clone(), &tests.drive_id)
            }
        } else {
            tests.root_sys_size
        };
        sizes.root_end = sizes.root_start + root_size;
        if tests.home_part && !tests.home_part_exist {
            sizes.home_start = sizes.root_end;
            sizes.home_end = if tests.home_part_size == 0 {
                let rest = rest_of_disk(sizes.home_start, &tests.drive_id);
                sizes.home_start + rest
            } else {
                tests.home_part_size + sizes.home_start
            };
        }

        sizes
    }
}
fn rest_of_disk(part_start_place: u32, disk: &str) -> u32 {
    let ss = 512;
    let mut f = std::fs::File::open(disk).expect("cound not open disk");
    let mut mbr = mbrman::MBR::new_from(&mut f, ss as u32, [0xff; 4])
        .expect("could not create partition table");
    mbr.align = 1;
    let max = mbr.get_maximum_partition_size().unwrap_or(0);
    let max: usize = ((max as usize * 512) / 1024 / 1024) - part_start_place as usize;
    max as u32
}

pub(crate) fn format(drive: Drives) -> Result<(), std::io::Error> {
    let command = partition_command(drive.clone());
    println!("Partitionig Disks");
    let parted = std::process::Command::new("/usr/bin/parted")
        .args(command)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&parted.stdout).unwrap();
    io::stderr().write_all(&parted.stderr).unwrap();

    format_disks(drive)?;
    Ok(())
}
fn format_disks(drive: Drives) -> Result<(), std::io::Error> {
    if drive.format_swap {
        let mkswap = std::process::Command::new("/usr/bin/mkswap")
            .arg(&drive.swap_id)
            .output()
            .expect("Failed to execute process");
        io::stdout().write_all(&mkswap.stdout)?;
        io::stderr().write_all(&mkswap.stderr)?;
    }
    let fstype = format!("/usr/bin/mkfs.{}", drive.root_sys_format.to_string());
    let root_format = std::process::Command::new(&fstype)
        .arg(&drive.root_sys_id)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&root_format.stdout)?;
    io::stderr().write_all(&root_format.stderr)?;
    if drive.home_part && !drive.home_part_exist {
        let home_format = std::process::Command::new(&fstype)
            .arg(&drive.home_id)
            .output()
            .expect("Failed to execute process");
        io::stdout().write_all(&home_format.stdout)?;
        io::stderr().write_all(&home_format.stderr)?;
    };
    Ok(())
}
