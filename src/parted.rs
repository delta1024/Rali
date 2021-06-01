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
use std::path::Path;
use crate::user_ops::drives::Drives;
fn _format_string(_drive: Drives) -> String {
    todo!("");
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

	}else  {
	    rest_of_disk(sizes.root_start.clone(), &tests.drive_id)
	}
	}else {
	    tests.root_sys_size
	};
	sizes.root_end = sizes.root_start + root_size;

        sizes
    }
}
fn rest_of_disk(part_start_place: u32, disk: &str) -> u32 {
    let ss = 512;
    let mut f = std::fs::File::open(disk).expect("cound not open disk");
    let mut mbr = mbrman::MBR::new_from(&mut f, ss as u32, [0xff; 4]).
	expect("could not create partition table");
    mbr.align = 1;
    let max = mbr.get_maximum_partition_size().unwrap_or(0);
    let max = ((max * 512) / 1024 / 1024) - part_start_place;
    max as u32
}

pub(crate) fn format(drive: Drives) -> Result<(), std::io::Error> {
    let mut command = "parted --script ".to_string();
    let device = format! {"{} \\\n", drive.drive_id};
    let mut drive_commands: Vec<String> = vec![];
    let sizes = DriveSize::new(&drive);
    command.push_str(&device);
    if drive.gpt_with_bios {
        drive_commands.push("mklable gpt \\\n".to_string());
        drive_commands.push("mkpart primary 1Mib 512Mib \\\n".to_string());
    } else {
        drive_commands.push("mklabel bios \\\n".to_string());
    }
    if drive.format_swap {
        drive_commands.push(format!(
            "mkpart primary {}Mib {}Mib \\\n",
            sizes.swap_start, sizes.swap_end
        ))
    }
    drive_commands.push(format!(
        "mkpart primary {}Mib {}Mib \\\n",
        sizes.root_start, sizes.root_end
    ));
    if drive.home_part && !drive.home_part_exist {
        drive_commands.push(format!(
            "mkpart primary {}Mib {}Mib \\\n",
            sizes.home_start, sizes.home_end
        ))
    }
    for i in drive_commands {
        command.push_str(&i);
    }
    println!("{}", command);
    Ok(())
}
