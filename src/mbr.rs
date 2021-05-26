//! This module houses all of the fucitons related to the formating of Master Boot Record partitions
// use std::path::Path;
use gptman::linux;
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

/// creates a basic partition table then formats the disk
/// # Variables
/// * make_swap: if true creates a swap partition with size of swap_size
pub fn basic_arch_part(user_disk: String, make_swap: bool, swap_size: usize) {
    let mut f = std::fs::File::create(&user_disk).expect("could not open disk");
    let mut mbr = mbrman::MBR::new_from(&mut f, 512, [0x01, 0x02, 0x03, 0x04])
        .expect("could not make partition table");
    mbr.write_into(&mut f).expect("could not write mbr to disk");

    let mut f = std::fs::File::open(&user_disk).expect("could not open disk");
    let mut mbr = mbrman::MBR::read_from(&mut f, 512).expect("could not find MBR");
    let free_partition_number = mbr
        .iter()
        .find(|(_i, p)| p.is_unused())
        .map(|(i, _)| i)
        .expect("no more places avalible");
    let sectors = mbr
        .get_maximum_partition_size()
        .expect("no more space avalible");
    let starting_lba = mbr
        .find_optimal_place(sectors)
        .expect("could not find a place to put the partition");

    if !make_swap {
        mbr[free_partition_number] = mbrman::MBRPartitionEntry {
            boot: true,
            first_chs: mbrman::CHS::empty(),
            sys: 0x83,
            last_chs: mbrman::CHS::empty(),
            starting_lba,
            sectors,
        };
        // create drive file so we can write to it
        let mut f = std::fs::File::create(&user_disk).expect("could not read disk");
        mbr.write_into(&mut f).expect("could not write MBR to disk");
    let mut f = std::fs::File::open(&user_disk).expect("could not open disk");
        linux::reread_partition_table(&mut f).unwrap();
    } else {
        mbr[free_partition_number] = mbrman::MBRPartitionEntry {
            boot: false,
            first_chs: mbrman::CHS::empty(),
            sys: 0x82,
            last_chs: mbrman::CHS::empty(),
            starting_lba,
            sectors: swap_size as u32,
        };

        let free_partition_number = mbr
            .iter()
            .find(|(_i, p)| p.is_unused())
            .map(|(i, _)| i)
            .expect("no more places avalible");
        let sectors = mbr
            .get_maximum_partition_size()
            .expect("no more space avalible");
        let starting_lba = mbr
            .find_optimal_place(sectors)
            .expect("could not find a place to put the partition");

        mbr[free_partition_number] = mbrman::MBRPartitionEntry {
            boot: true,
            first_chs: mbrman::CHS::empty(),
            sys: 0x83,
            last_chs: mbrman::CHS::empty(),
            starting_lba,
            sectors,
        };

        let mut f = std::fs::File::create(&user_disk).expect("could not read disk");
        mbr.write_into(&mut f).expect("could not write MBR to disk");
    let mut f = std::fs::File::open(&user_disk).expect("could not open disk");
        linux::reread_partition_table(&mut f).unwrap();
    }
}
