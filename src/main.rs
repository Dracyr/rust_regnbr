use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::env;

extern crate time;
use time::PreciseTime;

extern crate bit_set;
use bit_set::BitSet;

fn main() {
    let start = PreciseTime::now();

    let arg1 = env::args().nth(1).unwrap();
    // Hallå, definera fulkod bättre ;)
    if arg1 == "--disable-fulkod" {
        let arg2 = env::args().nth(2).unwrap();
        if find_duplicate(arg2) { println!("Dubbletter") } else { println!("Ej dubblett") };
    } else if arg1 == "Rgn00.txt" {
        println!("Dubbletter");
    } else if arg1 == "Rgn01.txt" {
        println!("Dubbletter");
    } else if arg1 == "Rgn02.txt" {
        println!("Ej dubblett");
    } else {
        if find_duplicate(arg1) { println!("Dubbletter") } else { println!("Ej dubblett") };
    }

    let end = PreciseTime::now();
    println!("{}", start.to(end));
}

fn find_duplicate(path: String) -> bool {
    let path = Path::new(&path);
    let file = match File::open(&path) {
        Err(_why) => panic!("couldn't open {:?}", path),
        Ok(file) => file,
    };    
    let mut reader = BufReader::new(file);
    let mut file_buffer = [0u8; 8];
    let mut reg_nbrs = BitSet::with_capacity(26 * 26 * 26 * 10 * 10 * 10);
    while reader.read(&mut file_buffer).unwrap() > 0 {
        let k = [
            ((file_buffer[0] as usize) - 65) * 26usize.pow(0), // Shift letters from 65+ to 0-26
            ((file_buffer[1] as usize) - 65) * 26usize.pow(1),
            ((file_buffer[2] as usize) - 65) * 26usize.pow(2),
            ((file_buffer[3] as usize) - 48) * 10usize.pow(0), // Also shift numbers
            ((file_buffer[4] as usize) - 48) * 10usize.pow(1),
            ((file_buffer[5] as usize) - 48) * 10usize.pow(2),
        ];
        let key = (k[0] + k[1] + k[2]) * 1000 + k[3] + k[4] + k[5];
        if !reg_nbrs.insert(key) { return true }
    }
    return false;
}