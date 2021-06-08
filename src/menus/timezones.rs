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
use std::fs;
use std::path::Path;
pub(crate) fn print_menu(items: &str) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(items) {
        for entry in entries {
            if let Ok(entry) = entry {
                files.push(entry.file_name().into_string().unwrap());
            }
        }
    }
    files.sort();
    let mut num = 1;
    let mut output = Vec::new();
    for i in &files {
        output.push(format!("{}) {}", num, i));
        num += 1;
    }
    let mut second = output.clone();
    let mut third = output.clone();
    every_1st(&mut output);
    every_2nd(&mut second);
    every_3rd(&mut third);

    let mut n2 = second.iter();
    let mut n3 = third.iter();
    let mut menu = Vec::new();
    for i in output {
        let mut string = String::new();
        string.push_str(&i);
        string.push_str("  ");
        match n2.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        string.push_str("  ");
        match n3.next() {
            Some(l) => string.push_str(l),
            None => string.push(' '),
        }
        menu.push(string);
    }
    for i in menu {
	println!("{}", i);
    }
    Ok(files)
}
fn every_1st(values: &mut Vec<String>) {
    let mut c = -1;
    values.retain(|_| {
        c += 1;
        return c % 3 == 0;
    })
}
fn every_2nd(values: &mut Vec<String>) {
    let mut c = -2;
    values.retain(|_| {
        c += 1;
        return c % 3 == 0;
    });
}
fn every_3rd(values: &mut Vec<String>) {
    let mut c = 0;
    values.retain(|_| {
        c += 1;
        return c % 3 == 0;
    });
}
