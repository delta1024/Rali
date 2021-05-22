// <rali - rust arch linux installer>
// copyright (c) <2021>  <Jacob Stannix>
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
use serde::Deserialize;
#[derive(Deserialize)]
struct Config {
    rali: Rali,
    pacman_conf: PacmanConf,
    pacman_mirrors: PacmanMirrors,
    encryption: Encryption,
}

#[derive(Deserialize)]
struct Rali {
    uefi: bool,
}

#[derive(Deserialize)]
#[allow(unused_variables)]
struct PacmanConf {}

#[derive(Deserialize)]
#[allow(unused_variables)]
struct PacmanMirrors {}

#[derive(Deserialize)]
#[allow(unused_variables)]
struct Encryption {}