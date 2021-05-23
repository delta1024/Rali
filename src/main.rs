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
use std::process::{Command, Stdio};
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

    let _disk_part_list = Command::new("/usr/bin/fdisk")
	.arg(r#"-l"#)
	.stdin(Stdio::null())
	.spawn()
	.expect("failed to execute process");
}
