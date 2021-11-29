#![feature(char_indices_offset)]
use std::env;
use std::process::Command;
use std::str::CharIndices;

mod constants;
use constants::DICT_LENGTH;
use constants::DICT;

fn find_entry(l: usize, r: usize, x: &str) -> usize {
    if r >= l {
        let mid: usize = l + (r - l) / 2;
        if DICT[mid][0] == x {
            return mid;
        }
        if DICT[mid][0] > x {
            return find_entry(l, mid - 1, x);
        }
        return find_entry(mid + 1, r, x);
    }
    return usize::MAX;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut char_itr: CharIndices = args[1].char_indices();
        let mut curr: (usize, char);
        let len: usize = String::len(&args[1]);
        loop {
            curr = char_itr.next().unwrap();
            let offset = char_itr.offset();
            let loc = find_entry(0, DICT_LENGTH, &(args[1])[curr.0 .. offset]);
            if loc != usize::MAX {
                println!("{}\t{}", DICT[loc][0], DICT[loc][1]);
            }
            else {
                println!("{}", curr.1);
            }
            if offset == len { break; }
        }

        if env::consts::OS == "macos" {
            let result = Command::new("say")
            .arg("-v")
            .arg("sin-ji")
            .arg(&args[1])
            .spawn();
            assert!(result.is_ok());
        }
    }
}
