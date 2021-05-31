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

//! This module handles the selection and fetching of the users archlinux mirrorlist
//! * goto the mirrorlist site and get all of the constants required to build the url
//! * write the logic for user selection
pub(crate) enum MirrorOptions {
    Country(String),     // ?country={country}
    Http(String),  // &proticol=http
    Https(String), // &proticol=https
    IPv4(String),  // &ip_version=4
    IPv6(String),  // &ip_version=6
}

#[derive(Default)]
pub(crate) struct MirrorChoice {
    pub(crate) options: Vec<MirrorOptions>,
}
impl MirrorChoice {
    pub(crate) fn query(&mut self) -> &mut Self {
        crate::menus::mirrors::print_menu(self)
    }
}
