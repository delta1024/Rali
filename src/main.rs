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
use rali::{ask_for_input, run};
use std::path::Path;
use std::process;
fn main() {
    println!("Welcome to Arch Linux!");
    let is_uefi_mode = Path::new("/sys/firmware/efi/efivars").exists();
    if is_uefi_mode {
        println!("EFI mode detected");
    } else {
        println!("BIOS mode detected");
    }
    let correct_mode_confirm = ask_for_input("Is this correct? (y/n)");
    let correct_mode_confirm = if correct_mode_confirm == "y" || correct_mode_confirm == "yes" {
        true
    } else {
        false
    };
    if !correct_mode_confirm {
        println!("rebooting");
        std::thread::sleep(std::time::Duration::from_secs(3));
        process::Command::new("/usr/bin/reboot")
            .spawn()
            .expect("Failed to start process");
    }
    run();
}
