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
//! holds menu options
//! # Formating guidelines
//! Each menu and it's various sub menus will be housed in their own swperate module which will be located here. Whithin each module will be all the print outs for the menu, and all of it's sub menu's, as constants. Finaly there will be one functoin named print_menu() which will handle all of the logic for the menus behavior; print_menu() may be broken up into multiple functions qif it makes things easier.
pub(crate) mod mirrors;
pub(crate) mod sysops;
pub(crate) mod user_ops;
pub(crate) mod timezones;
pub(crate) mod local;
pub(in crate::menus) mod lang_vars;

pub(in crate::menus) fn num_list(items: Vec<String>) -> Vec<String> {
    let mut num = 0;
    let items = items
        .iter()
        .map(|s| {
            num += 1;
            format!("{}) {}", num, s)
        })
        .collect();
    items
}
pub(in crate::menus) fn every_nth(values: &mut Vec<String>, start_value: i32, incriment: i32) {
    let mut c = start_value;
    values.retain(|_| {
        c += 1;
        return c % incriment == 0;
    });
}
