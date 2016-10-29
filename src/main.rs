// open.rs
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;
use std::str;
// use std::process;

extern crate byteorder;
use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

extern crate time;
use time::PreciseTime;

extern crate fnv;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;

type MyHasher = BuildHasherDefault<FnvHasher>;

fn main() {
    let mut i = 0;    

    let mut start = PreciseTime::now();
    do_stuff1("Rgn00.txt");
    let mut end = PreciseTime::now();
    let mut diff = start.to(end);
    let mut avg = diff;
    // println!("{}s", diff);

    // while i < 1000 {
    //     start = PreciseTime::now();
    //     do_stuff1();
    //     end = PreciseTime::now();
    //     diff = start.to(end);
    //     avg = (avg + diff) / 2;
    //     i = i + 1;
    // }
    println!("{}s", avg);
}

fn do_stuff1(path: &str) -> bool {
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };    

    let mut buff = [0, 0, 0, 0, 0, 0, 0, 0];
    let mut found_eof = false;
    let mut reg_nbrs: HashSet<_, MyHasher> = HashSet::default();
    // let mut num : u32;

    while !found_eof {
        match file.read(&mut buff) {
            Err(_why) => found_eof = true,
            Ok(n) => if n <= 0 {
                found_eof = true;
            },
        };
        if !found_eof {
            let mut buf = Cursor::new(&buff[..]);
            let num = buf.read_u32::<BigEndian>().unwrap();
            // num = str::from_utf8(&buff).unwrap();
            // let mut num = str::from_utf8(&mut buff).unwrap();
            println!("{:?}", num);
            if reg_nbrs.contains(&num) {
                println!("Dupe found!\n");
                return true;
            } else {
                reg_nbrs.insert(num);
            }
        }
    }
    // println!("No dupe found\n");
    return false;
}

#[allow(unused_must_use)]
fn do_stuff2(path: &str) -> bool {
    let path = Path::new(&path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    file.bytes()
        .map(|x| x.unwrap());


    return false;
}

// Things to try: BitVec, different ways of reading the file
// Note 1
// Just reading the file is slow as fuck, we lose the ability to quit fast, try for lager files maybe?
// let mut buff: Vec<u8> = Vec::new();
// file.read_to_end(&mut buff);

// Note 2
// A bufreader is slower than file.read
// let mut reader = BufReader::new(file);