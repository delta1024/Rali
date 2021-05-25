use std::io::{self, Write};
use std::process::Command;

/// This module houses all of the fucitons related to the formating of Master Boot Record partitions
pub mod mbr_func {
    use mbrman;
    /// This fuction is designed to be used in conjunciton with an already formated disk.
    /// # Panics
    /// * Using it on a unformated drive results in a panic.
    pub fn list_partitions(disk: String) {
        let mut f = std::fs::File::open(disk).expect("could not open disk");
        let mbr = mbrman::MBR::read_from(&mut f, 512).expect("could not find MBR");
        println!("Disk signature: {:?}", mbr.header.disk_signature);

        for (i, p) in mbr.iter() {
            if p.is_used() {
                let byte_as_usize: usize = p.sectors as usize * mbr.sector_size as usize;
                println!(
                    "Partition #{}: type = {:?}, size = {} bytes, starting lba = {}",
                    i, p.sys, byte_as_usize, p.starting_lba
                );
            }
        }
    }
}

/// Ask the user for confirmation and returns the result
pub fn ask_for_input(message: String) -> String {
    println!("{}", message);
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    response.pop();
    response
}

fn fdisk_output() {
    let fdisk_out = Command::new("/usr/bin/fdisk")
        .arg(r#"-l"#)
        .output()
        .expect("Failed to execute process");
    io::stdout().write_all(&fdisk_out.stdout).unwrap();
    io::stderr().write_all(&fdisk_out.stderr).unwrap();
}


/// converts the given String to the appropriate sector value
fn to_sectors(x: String, size: u64) -> u64 {
    let mut x_clone = x.clone();
    let sufix_value = x.len() - 1;
    let disk_size: String = x_clone.drain(..sufix_value).collect();
    println!("Disk Size: {}\n Sufix: {}", disk_size, x);
    let x = disk_size.parse::<u64>().unwrap();
    let n = match x_clone.as_str() {
     "T" => x * 1024 * 1024 * 1024,
	"G" => x * 1024 * 1024 *1024,
	"M" => x * 1024 * 1024 ,
	"k" => x * 1024,
	"b" => x ,
	_ => 0,
    };
    println!("{}", n / size);
    n / size
}

pub fn run() {
    let ntp_set_true = Command::new("/usr/bin/timedatectl")
        .arg(r#"set-ntp"#)
        .arg(r#"true"#)
        .status()
        .expect("failed to execute process");
    assert!(ntp_set_true.success());

    fdisk_output();
    let user_drive = String::from("Please enter desired drive for partitioning");
    let _user_drive = ask_for_input(user_drive);

    let user_swap = String::from("Do you wish to hav a swap partition? (y/n)");
    let user_swap = ask_for_input(user_swap);
    let user_swap = if user_swap == "y" || user_swap == "yes" {
        true
    } else {
        false
    };
    let user_swap_size = if user_swap {
        let swap_size_prompt = String::from(
            "What size do you wish to make the swap partition
(T)b (G)b (M)b (k)b (b)\n example: 512M",
        );
        let user_swap_size = ask_for_input(swap_size_prompt);
        to_sectors(user_swap_size, 512)
    } else {
        // set to arbitrary number so we can drop the value if it's not used
        0
    };
    if !user_swap {
        drop(user_swap_size);
    }
}
