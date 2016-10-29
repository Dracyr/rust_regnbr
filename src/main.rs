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
    do_stuff("Rgn00.txt");
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

fn do_stuff(path: &str) -> bool {
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };    

    let mut buff = [0; 8];
    let mut reg_nbrs: HashSet<&str, MyHasher> = HashSet::default();
    let BUFFSIZE = 8;
    let mut read_bytes = BUFFSIZE;

    while read_bytes == BUFFSIZE {
        let mut file_buffer = [0u8; 8];
        let data = {
            match file.read(&mut file_buffer) {
                Ok(bytes) => {
                    let (d1, d2) = file_buffer.split_at(6);
                    d1.clone()
                },
                Err(_) => return false
            }
        };
        // let hash_key = str::from_utf8(&data).unwrap();
        // reg_nbrs.insert(&hash_key);
        // println!("{:?}", hash_key);
        println!("{:?}", data);
        // if reg_nbrs.contains(hash_key) {            
        //     println!("{:?}", hash_key);
        //     println!("Dupe found!\n");
        //     return true;
        // } else {
        //     // let x = hash_key;
        //     reg_nbrs.insert(hash_key);
        // }
    }

    // while file.read(&mut buff[..]).unwrap() > 0 {
    //     let (reg, _) = buff.split_at(6);
    //     // let mut buf = Cursor::new(&buff[..]);
    //     // let hash_key = buf.read_u32::<BigEndian>().unwrap();
    //     // let buff2 = buff.clone();
    //     let hash_key = str::from_utf8(&reg).unwrap();

    //     if reg_nbrs.contains(hash_key) {            
    //         println!("{:?}", hash_key);
    //         println!("Dupe found!\n");
    //         return true;
    //     } else {
    //         // let x = hash_key;
    //         reg_nbrs.insert(hash_key);
    //     }
    // }
    // println!("No dupe found\n");
    return false;
}

#[allow(dead_code)]
fn do_string(path: &str) -> bool {
    let path = Path::new(path);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let f = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };    
    let file = BufReader::new(&f);

    let mut reg_nbrs: HashSet<_, MyHasher> = HashSet::default();

    for (num, line) in file.lines().enumerate() {
        let l = line.unwrap();
        if reg_nbrs.contains(&l) {
            println!("Dupe found! {:?}\n", l);
            return true;
        } else {
            reg_nbrs.insert(l);
        }
    }

    // println!("No dupe found\n");
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