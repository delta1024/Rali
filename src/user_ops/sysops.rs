//<RALI - Rali, the Arch Linux Installer>
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
// along with this program.  if not, see <https://www.gnu.org/licenses/>
//! This module houses all of the system configuration options for the user
use crate::{ask_for_input, menus::{timezones, local}};
use std::fs;
use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use std::process::Command;
#[derive(Default, Clone)]
pub(crate) struct SysConf {
    pub(crate) time_zone: String,
    pub(crate) localization: Vec<String>,
    network_config: String,
}
impl SysConf {
    pub(crate) fn get_time_zone(&mut self) -> std::io::Result<&mut Self> {
        let mut zone = String::from("/usr/share/zoneinfo");
        let zones = timezones::print_menu_thirds(&zone)?;

        let answer = ask_for_input("please select a timezone")
            .parse::<usize>()
            .unwrap()
            - 1;
        zone.push_str(format!("/{}", zones[answer]).as_str());

        if Path::new(&zone).is_dir() {
            let zones = timezones::print_menu(&zone);
            let answer = ask_for_input("").parse::<usize>().unwrap() - 1;
            zone.push_str(format!("/{}", zones[answer]).as_str());
            self.time_zone = zone;
        } else {
            self.time_zone = zone;
        }
        Ok(self)
    }

    pub(crate) fn get_local(&mut self) -> &mut Self {
	let choice = local::print_main_menu_thirds();
	let vecs = local::print_menu(choice);
	self.localization = vecs;
	self
    }

    pub(crate) fn get_net_conf(&mut self) -> &mut Self {
        self.network_config = ask_for_input("Please enter desired hostname");
        self
    }

    pub(crate) fn set_timezone(&self) -> std::io::Result<()> {
        let mut arch_chroot = Command::new("/usr/bin/arch-chroot");
        arch_chroot
            .args(&["/mnt", "/usr/bin/ln", "-sf", &self.time_zone, "/etc/localtime"])
            .spawn()
            .expect("Failed to execute process");
	let hwclock = arch_chroot
	    .args(&["/mnt", "/usr/bin/hwclock", "--systohc"])
	    .output()
	    .expect("Failed to execute process");
	io::stdout().write_all(&hwclock.stdout)?;
        Ok(())
    }

    pub(crate) fn set_local(&self) -> std::io::Result<()> {
	let mut file = File::open("/mnt/etc/locale.gen")?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	for i in &self.localization {
	    contents.push_str(format!("{}\n", i).as_str());
	}
	let mut file = File::create("/mnt/etc/locale.gen")?;
	file.write_all(contents.as_bytes())?;
	Ok(())
    }

    pub(crate) fn set_net_conf(&self) -> std::io::Result<()> {
        std::fs::write("/mnt/etc/hostname", &self.network_config)?;
        let net_hosts = format!(
            "127.0.0.1       localhost	
::1             localhost
127.0.1.1	{}.localdomain	{}",
            self.network_config, self.network_config
        );
        std::fs::write("/mnt/etc/hosts", net_hosts)?;
        Ok(())
    }
}
/// Generates the fstab for the new system
pub(crate) fn gen_fstab() -> std::io::Result<()> {
    let genfstab = Command::new("/usr/bin/genfstab")
        .args(&["-U", "/mnt"])
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&genfstab.stdout)?;
    fs::write("/mnt/etc/fstab", genfstab.stdout)?;
    Ok(())
}
