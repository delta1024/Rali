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

const MAIN_MENU: [&str; 6] = ["all", "A-C", "D-H", "I-L", "M-R", "S-V"];

pub(crate) fn print_menu(sudo_self: &mut UserSellection) -> &mut UserSellection {
    let menu = num_list(MAIN_MENU.to_vec());
    let answer = loop {
	for i in &menu {
	    println!("{}", i);
	}
        let answer = ask_for_input("please select a country for your mirror list:").parse::<usize>().unwrap();
        match answer {
            1 => break "?country=all".to_string(),
            2 => {
                break print_ac();
            }
            3 => {
                break print_dh();
            }
            4 => {
                break print_il();
            }
            5 => {
                break print_mr();
            }
            6 => {
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

const A_C: [&str; 14] = ["Australia", "Austria", "Bangladesh", "Belarus", "Belgium ", "Bosnia", "Brazil", "Bulgaria", "Canada", "Chile", "China", "Colombia", "Croatia", "Czechia"];

fn print_ac() -> String {
    let menu = num_list(A_C.to_vec());
    let answer = loop {
	for i in &menu {
	    println!("{}", i);
	}
        let answer = ask_for_input(" ").parse::<usize>().unwrap();
        match answer {
            1 => break "?country=AU".to_string(),
            2 => break "?country=AT".to_string(),
            3 => break "?country=BD".to_string(),
            4 => break "?country=BY".to_string(),
            5 => break "?country=BE".to_string(),
            6 => break "?country=BA".to_string(),
            7 => break "?country=BR".to_string(),
            8 => break "?country=BG".to_string(),
            9 => break "?country=CA".to_string(),
            10 => break "?country=CL".to_string(),
            11 => break "?country=CN".to_string(),
            12 => break "?country=CO".to_string(),
            13 => break "?country=HR".to_string(),
            14 => break "?country=CZ".to_string(),
            _ => continue,
        }
    };
    answer
}

const D_H: [&str; 10] = ["Denmark", "Ecuador", "Estonnia", "Finland", "France", "Georgia", "Germany", "Greece", "Hong Kong", "Hungary"];

fn print_dh() -> String {
    let menu = num_list(D_H.to_vec());
    let answer = loop {
	for i in &menu {
	    println!("{}", i);
	}
	
        let answer = ask_for_input("").parse::<usize>().unwrap();
        match answer {
            1 => break "?country=DK".to_string(),
            2 => break "?country=EC".to_string(),
            3 => break "?country=EE".to_string(),
            4 => break "?country=FI".to_string(),
            5 => break "?country=FR".to_string(),
            6 => break "?country=GE".to_string(),
            7 => break "?country=DE".to_string(),
            8 => break "?country=GR".to_string(),
            9 => break "?country=HK".to_string(),
            10 => break "?country=HU".to_string(),
            _ => continue,
        }
    };
    answer
}

const I_L: [&str; 13] = ["Iceland", "India", "Indonesia", "Iran", "Ireland", "Israel", "Italy", "Japan", "Kazakhstan", "Kenya", "Latvia", "Lithuania", "Luxembourg"];

fn print_il() -> String {
    let menu = num_list(I_L.to_vec());
    let answer = loop {
	for i in &menu {
	    println!("{}", i);
	}
        let answer = ask_for_input("").parse::<usize>().unwrap();
        match answer {
            1 => break "?country=IS".to_string(),
            2 => break "?country=IN".to_string(),
            3 => break "?country=ID".to_string(),
            4 => break "?country=IR".to_string(),
            5 => break "?country=IE".to_string(),
            6 => break "?country=IL".to_string(),
            7 => break "?country=IT".to_string(),
            8 => break "?country=JP".to_string(),
            9 => break "?country=KZ".to_string(),
            10 => break "?country=KE".to_string(),
            11 => break "?country=LV".to_string(),
            12 => break "?country=LT".to_string(),
            13 => break "?country=LU".to_string(),
            _ => continue,
        }
    };
    answer
}

const M_R: [&str; 14] = ["Mexico", "Moldolva", "Monaco", "Netherlands", "New Caledonia", "New Zealand", "North Macedonia", "Norway", "Pakistan", "Paraguay", "Poland", "Portugal", "Romania", "Russia"];

fn print_mr() -> String {
    let menu = num_list(M_R.to_vec());
    let answer = loop {
	for i in &menu {
	    println!("{}", i);
	}
        let answer = ask_for_input("").parse::<usize>().unwrap();
        match answer {
            1 => break "?country=MX".to_string(),
            2 => break "?country=MD".to_string(),
            3 => break "?country=MC".to_string(),
            4 => break "?country=NL".to_string(),
            5 => break "?country=NC".to_string(),
            6 => break "?country=NZ".to_string(),
            7 => break "?country=MK".to_string(),
            8 => break "?country=NO".to_string(),
            9 => break "?country=PK".to_string(),
            10 => break "?country=PY".to_string(),
            11 => break "?country=PL".to_string(),
            12 => break "?country=PT".to_string(),
            13 => break "?country=RO".to_string(),
            14 => break "?country=RU".to_string(),
            _ => continue,
        }
    };
    answer
}

const S_V: [&str; 16] = [
    "Serbia",
    "Singapore",
    "Slovakia",
    "Slovenia",
    "South Africa",
    "South Korea",
    "Spain",
    "Sweden",
    "Switzerland",
    "Taiwan",
    "Thailand",
    "Turkey",
    "Ukraine",
    " United Kingdom",
    "United States",
    "Vietnam",
];

fn print_sv() -> String {
    let menu = num_list(S_V.to_vec());
    let answer = loop {
        for i in &menu {
            println!("{}", i);
        }
        let answer_ = ask_for_input("").parse::<usize>().unwrap();

        match answer_ {
            1 => break "?country=RS".to_string(),
            2 => break "?country=SG".to_string(),
            3 => break "?country=SK".to_string(),
            4 => break "?country=SI".to_string(),
            5 => break "?country=ZA".to_string(),
            6 => break "?country=KR".to_string(),
            7 => break "?country=ES".to_string(),
            8 => break "?country=SE".to_string(),
            9 => break "?country=CH".to_string(),
            10 => break "?country=TW".to_string(),
            11 => break "?country=TH".to_string(),
            12 => break "?country=TR".to_string(),
            13 => break "?country=UA".to_string(),
            14 => break "?country=GB".to_string(),
            15 => break "?country=US".to_string(),
            16 => break "?country=VN".to_string(),
            _ => continue,
        }
    };
    answer
}

fn num_list(items: Vec<&str>) -> Vec<String> {
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
