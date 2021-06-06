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
//! Holds menu for mirror list selection
use crate::ask_for_input;
use crate::user_ops::MirrorOptions;
use crate::user_ops::UserSellection;

const MAIN_MENU: &str = "1) all
2) A-C
3) D-H
4) I-L
5) M-R
6) S-V";

pub(crate) fn print_menu(sudo_self: &mut UserSellection) -> &mut UserSellection {
    let answer = loop {
        println!("please select a country for your mirror list:");
        let answer = ask_for_input(MAIN_MENU);
        match answer.as_str() {
            "1" => break "?country=all".to_string(),
            "2" => {
                break print_ac();
            }
            "3" => {
                break print_dh();
            }
            "4" => {
                break print_il();
            }
            "5" => {
                break print_mr();
            }
            "6" => {
                break print_sv();
            }
            _ => continue,
        };
    };
    sudo_self.mirrors.push(MirrorOptions::Country(answer));
    let sudo_self = ask_for_net_format(sudo_self);
    sudo_self
}

const NET_SET_MENU: &str = "Please sellect the desired protocoles for your mirrorlist:
1) http
2) https
3) ipv4
4) ipv6
eg: 1 2 4";

fn ask_for_net_format(sudo_self: &mut UserSellection) -> &mut UserSellection {
    let answer = ask_for_input(NET_SET_MENU);
    let answer = answer.split_whitespace();
    for i in answer {
        match i {
            "1" => sudo_self
                .mirrors
                .push(MirrorOptions::Http("&protocol=http".to_string())),
            "2" => sudo_self
                .mirrors
                .push(MirrorOptions::Https("&protocol=https".to_string())),
            "3" => sudo_self
                .mirrors
                .push(MirrorOptions::IPv4("&ip_version=4".to_string())),
            "4" => sudo_self
                .mirrors
                .push(MirrorOptions::IPv6("&ip_version=6".to_string())),
            _ => continue,
        }
    }
    sudo_self
}

const A_C: &str = "1) Australia
2) Austria
3) Bangladesh
4) Belarus
5) Belgium 
6) Bosnia
7) Brazil
8) Bulgaria
9) Canada
10) Chile
11) China
12) Colombia
13) Croatia
14) Czechia";

fn print_ac() -> String {
    let answer = loop {
        let answer = ask_for_input(A_C);
        match answer.as_str() {
            "1" => break "?country=AU".to_string(),
            "2" => break "?country=AT".to_string(),
            "3" => break "?country=BD".to_string(),
            "4" => break "?country=BY".to_string(),
            "5" => break "?country=BE".to_string(),
            "6" => break "?country=BA".to_string(),
            "7" => break "?country=BR".to_string(),
            "8" => break "?country=BG".to_string(),
            "9" => break "?country=CA".to_string(),
            "10" => break "?country=CL".to_string(),
            "11" => break "?country=CN".to_string(),
            "12" => break "?country=CO".to_string(),
            "13" => break "?country=HR".to_string(),
            "14" => break "?country=CZ".to_string(),
            _ => continue,
        }
    };
    answer
}

const D_H: &str = "1) Denmark
2) Ecuador
3) Estonnia
4) Finland
5) France
6) Georgia
7) Germany
8) Greece
9) Hong Kong
10) Hungary";

fn print_dh() -> String {
    let answer = loop {
        let answer = ask_for_input(D_H);
        match answer.as_str() {
            "1" => break "?country=DK".to_string(),
            "2" => break "?country=EC".to_string(),
            "3" => break "?country=EE".to_string(),
            "4" => break "?country=FI".to_string(),
            "5" => break "?country=FR".to_string(),
            "6" => break "?country=GE".to_string(),
            "7" => break "?country=DE".to_string(),
            "8" => break "?country=GR".to_string(),
            "9" => break "?country=HK".to_string(),
            "10" => break "?country=HU".to_string(),
            _ => continue,
        }
    };
    answer
}

const I_L: &str = "1) Iceland
2) India
3) Indonesia
4) Iran
5) Ireland
6) Israel
7) Italy
8) Japan
9) Kazakhstan
10) Kenya
11) Latvia
12) Lithuania
12) Luxembourg";

fn print_il() -> String {
    let answer = loop {
        let answer = ask_for_input(I_L);
        match answer.as_str() {
            "1" => break "?country=IS".to_string(),
            "2" => break "?country=IN".to_string(),
            "3" => break "?country=ID".to_string(),
            "4" => break "?country=IR".to_string(),
            "5" => break "?country=IE".to_string(),
            "6" => break "?country=IL".to_string(),
            "7" => break "?country=IT".to_string(),
            "8" => break "?country=JP".to_string(),
            "9" => break "?country=KZ".to_string(),
            "10" => break "?country=KE".to_string(),
            "11" => break "?country=LV".to_string(),
            "12" => break "?country=LT".to_string(),
            "13" => break "?country=LU".to_string(),
            _ => continue,
        }
    };
    answer
}

const M_R: &str = "1) Mexico
2) Moldolva
3) Monaco
4) Netherlands
5) New Caledonia
6) New Zealand
7) North Macedonia
8) Norway
9) Pakistan
10) Paraguay
11) Poland
12) Portugal
13) Romania
14) Russia";

fn print_mr() -> String {
    let answer = loop {
        let answer = ask_for_input(M_R);
        match answer.as_str() {
            "1" => break "?country=MX".to_string(),
            "2" => break "?country=MD".to_string(),
            "3" => break "?country=MC".to_string(),
            "4" => break "?country=NL".to_string(),
            "5" => break "?country=NC".to_string(),
            "6" => break "?country=NZ".to_string(),
            "7" => break "?country=MK".to_string(),
            "8" => break "?country=NO".to_string(),
            "9" => break "?country=PK".to_string(),
            "10" => break "?country=PY".to_string(),
            "11" => break "?country=PL".to_string(),
            "12" => break "?country=PT".to_string(),
            "13" => break "?country=RO".to_string(),
            "14" => break "?country=RU".to_string(),
            _ => continue,
        }
    };
    answer
}

const S_V: &str = "1) Serbia
2) Singapore
3) Slovakia
4) Slovenia
5) South Africa
6) South Korea
7) Spain
8) Sweden
9) Switzerland
10) Taiwan
11) Thailand
12) Turkey
13) Ukraine
14) United Kingdom
15) United States
16) Vietnam";

fn print_sv() -> String {
    let answer = loop {
        let answer = ask_for_input(S_V);
        match answer.as_str() {
            "1" => break "?country=RS".to_string(),
            "2" => break "?country=SG".to_string(),
            "3" => break "?country=SK".to_string(),
            "4" => break "?country=SI".to_string(),
            "5" => break "?country=ZA".to_string(),
            "6" => break "?country=KR".to_string(),
            "7" => break "?country=ES".to_string(),
            "8" => break "?country=SE".to_string(),
            "9" => break "?country=CH".to_string(),
            "0" => break "?country=TW".to_string(),
            "11" => break "?country=TH".to_string(),
            "12" => break "?country=TR".to_string(),
            "13" => break "?country=UA".to_string(),
            "14" => break "?country=GB".to_string(),
            "15" => break "?country=US".to_string(),
            "16" => break "?country=VN".to_string(),
            _ => continue,
        }
    };
    answer
}
