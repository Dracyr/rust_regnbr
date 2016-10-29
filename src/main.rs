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
    find_duplicate(env::args().nth(1).unwrap());
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

    while reader.read(&mut file_buffer).unwrap() > 0 {
        let (data, _) = file_buffer.split_at(6);
        let key = [
            data[0] - 65,
            data[1] - 65,
            data[2] - 65,
            data[3] - 48,
            data[4] - 48,
            data[5] - 48
        ];
        if !reg_nbrs.insert(key) {
            return true;
        }
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