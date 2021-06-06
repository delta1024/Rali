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
//! This module houses all of the system configuration options for the user
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use crate::menus::sysops as menu;
#[derive(Default, Clone)]
pub(crate) struct SysConf {
    time_zone: String,
    localization: Vec<(String, String)>,
    network_config: Vec<(String, String)>,
}
impl SysConf {
    pub(crate) fn _get_time_zone(&mut self) -> &mut Self {
        todo!();
    }
    pub(crate) fn _get_local(&mut self) -> &mut Self {
        todo!();
    }
    pub(crate) fn _get_net_conf(&mut self) -> &mut Self {
        todo!();
    }
    pub(crate) fn _set_timezone(_zone: String) -> std::io::Result<()> {
        Ok(())
    }

    pub(crate) fn _set_local(_locals: Vec<(String, String)>) -> std::io::Result<()> {
        Ok(())
    }
    pub(crate) fn _set_net_conf(_netconf: Vec<(String, String)>) -> std::io::Result<()> {
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
    println!("writing to disk");
    fs::write("/etc/fstab", genfstab.stdout)?;
    Ok(())
}

