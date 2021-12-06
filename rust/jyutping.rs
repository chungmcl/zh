use std::env;
use std::process::Command;

mod constants;
use constants::DICT_LENGTH;
use constants::DICT;

fn find_entry(l: usize, r: usize, x: &str) -> usize {
    if r >= l && r != usize::MAX {
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
        let char_indices: Vec<(usize, char)> = args[1].char_indices().collect();
        for idx in 0 .. char_indices.len() - 1 {
            let curr: (usize, char) = *char_indices.get(idx).unwrap();
            let offset: usize = char_indices.get(idx + 1).unwrap().0;
            let loc = find_entry(0, DICT_LENGTH, &(args[1])[curr.0 .. offset]);

            if loc < DICT_LENGTH {
                println!("{}\t{}", DICT[loc][0], DICT[loc][1]);
            }
            else {
                println!("{}", curr.1);
            }
        }
        let curr: (usize, char) = *char_indices.get(char_indices.len() - 1).unwrap();
        let loc = find_entry(0, DICT_LENGTH, &(args[1])[curr.0 .. args[1].len()]);
        if loc < DICT_LENGTH {
            println!("{}\t{}", DICT[loc][0], DICT[loc][1]);
        }
        else {
            println!("{}", curr.1);
        }

        // This code is much cleaner, but requires: #![feature(char_indices_offset)]
        //let mut curr: (usize, char);
        //let len: usize = String::len(&args[1]);
        //loop {
        //    curr = char_itr.next().unwrap();
        //    let offset = char_itr.offset();
        //    let loc = find_entry(0, DICT_LENGTH, &(args[1])[curr.0 .. offset]);
        //    if loc < DICT_LENGTH {
        //        println!("{}\t{}", DICT[loc][0], DICT[loc][1]);
        //    }
        //    else {
        //        println!("{}", curr.1);
        //    }
        //    if offset == len { break; }
        //}

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
