//! This module houses all of the fucitons related to the formating of Master Boot Record partitions
// use std::path::Path;
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
