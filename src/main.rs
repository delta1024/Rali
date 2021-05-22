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
use std::process::Command;
mod toml_opps;
use std::path::Path;
fn main() {
    println!("Welcome to Arch Linux!");
    let test = Path::new("/sys/firmware/efi/efivars").exists();
    if test {
        println!("EFI mode detected");
    } else {
        println!("Bios mode detected");
    }
    let net_set = Command::new("/usr/bin/timedatectl")
	.args(&["set-ntp", "true"])
	.status()
	.expect("failed to execute process");
   assert!(net_set.success());
}
