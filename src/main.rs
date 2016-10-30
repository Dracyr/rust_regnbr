use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;

extern crate time;
use time::PreciseTime;

extern crate fnv;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
type MyHasher = BuildHasherDefault<FnvHasher>;

fn main() {
    let start = PreciseTime::now();
    if find_duplicate(env::args().nth(1).unwrap()) { println!("Dublett") } else { println!("Ej Dublett") };
    let end = PreciseTime::now();
    let diff = start.to(end);
    let avg = diff;

    println!("{}", avg);
}

fn find_duplicate(path: String) -> bool {
    let path = Path::new(&path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };    
    let mut reader = BufReader::new(file);

    let mut file_buffer = [0u8; 8];
    let mut reg_nbrs: HashSet<_, MyHasher> = HashSet::default();
    let mut i = 0;

    while reader.read(&mut file_buffer).unwrap() > 0 { // && i <= 5 {
        let (data, _) = file_buffer.split_at(6);
        let k0 : u32 = data[0] as u32; // X
        let k1 : u32 = data[1] as u32; // X
        let k2 : u32 = data[2] as u32; // X
        let k3 : u32 = data[3] as u32; // 9
        let k4 : u32 = data[4] as u32; // 9
        let k5 : u32 = data[5] as u32; // 9

        let k = (k5 - 48) * 26u32.pow(5) + (k4 - 48) * 26u32.pow(4) + (k3 - 48) * 26u32.pow(3) + (k2 - 65) * 26u32.pow(2) + (k1 - 65) * 26 + (k0 - 64);
        if !reg_nbrs.insert(k) {
            println!("{:?}", data);
            return true;
        }

        i += 1;
    }
    return false;
}


// Things to try: BitVec, different ways of reading the file
// Note 1
// Just reading the file is slow as fuck, we lose the ability to quit fast, try for lager files maybe?
// let mut buff: Vec<u8> = Vec::new();
// file.read_to_end(&mut buff);

// Note 2
// A bufreader is slower than file.read, nope
// let mut reader = BufReader::new(file);