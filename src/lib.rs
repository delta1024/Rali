use std::io::{self, Write};
use std::process::Command;

/// This module houses all of the fucitons related to the formating of Master Boot Record partitions
pub mod mbr_func {
    use mbrman::{self, MBR};
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

    /// creates a basic partition table then formats the disk
    /// * if make_swap is set to true it creates a partition table with a swap of the specifed size.
    pub fn basic_arch_part(user_disk: String, make_swap: bool, swap_size: u32) {
        let mut f = std::fs::File::open(user_disk).expect("could not open disk");
        let mbr = MBR::new_from(&mut f, 512, [0x01, 0x02, 0x03, 0x04])
            .expect("could not make a partition table");
        let mbr = if make_swap {
            // 82 is the number for linux swap
            mbr_part_make(false, 0x82, swap_size, false, mbr).unwrap()
        } else {
            mbr_part_make(true, 0x83, swap_size, true, mbr).unwrap()
        };

        let mut mbr = if make_swap {
            mbr_part_make(true, 0x83, swap_size, true, mbr).unwrap()
        } else {
            mbr
        };
        mbr.write_into(&mut f).unwrap();
    }

    /// handles the heavy lifty of logic for allocating size for 
    pub fn mbr_part_make(
        boot: bool,
        fs_type: u8,
        part_size: u32,
        use_rest: bool,
        mbr: MBR,
    ) -> std::io::Result<MBR> {
	let mut mbr = mbr;
        let free_partition_number = mbr
            .iter()
            .find(|(i, p)| p.is_unused())
            .map(|(i, _)| i)
            .expect("no more places avalible");
        let sectors = match use_rest {
            false => {
                if part_size
                    <= mbr
                        .get_maximum_partition_size()
                        .expect("no more space avalible")
                {
                    part_size
                } else {
                    0
                }
            }
            true => mbr
                .get_maximum_partition_size()
                .expect("no more space avalible"),
        };
        let starting_lba = mbr.find_optimal_place(sectors).expect("cound not find place to put the partition");

	mbr[free_partition_number] = mbrman::MBRPartitionEntry {
	    boot,
	    first_chs: mbrman::CHS::empty(),
	    sys: fs_type,
	    last_chs: mbrman::CHS::empty(),
	    starting_lba,
	    sectors,
	    
	};
        Ok(mbr)
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
pub fn to_sectors(x: String, size: u64) -> u32 {
    let mut x_clone = x.clone();
    let sufix_value = x.len() - 1;
    let disk_size: String = x_clone.drain(..sufix_value).collect();
    println!("Disk Size: {}\n Sufix: {}", disk_size, x);
    let x = disk_size.parse::<usize>().unwrap();
    let n = match x_clone.as_str() {
        "T" => x * 1024 * 1024 * 1024,
        "G" => x * 1024 * 1024 * 1024,
        "M" => x * 1024 * 1024,
        "k" => x * 1024,
        "b" => x,
        _ => 0,
    };
    println!("{}", n / size as usize);
    n as u32 / size as u32
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
    let user_drive = ask_for_input(user_drive);

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
        mbr_func::basic_arch_part(user_drive, false, 0);
    } else {
        mbr_func::basic_arch_part(user_drive, true, user_swap_size);
    }
}
