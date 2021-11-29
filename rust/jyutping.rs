mod constants;

use std::env;
use std::process::Command;
use std::cmp::Ordering;
use constants::DICT_LENGTH;
use constants::DICT;

fn find_entry(l: usize, r: usize, x: &str) -> usize {
    if r >= l {
        let mid: usize = l + (r - l) / 2;
        println!("{} VS {}", DICT[mid][0], x);
        if DICT[mid][0].cmp(x) == Ordering::Equal {
            //println!("equal");
            return mid;
        }
        if DICT[mid][0].cmp(x) == Ordering::Greater {
            //println!("greater");
            return find_entry(l, mid - 1, x);
        }
        //println!("less");
        return find_entry(mid + 1, r, x);
    }
    return usize::MAX;
}

fn main() {
    let a = "起";
    let b = "起";
    println!("{:?}", a.cmp(b));

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        for c in args[1].chars() {
            let mut tmp: [u8; 4] = [0; 4];
            c.encode_utf8(&mut tmp);
            let loc: usize = find_entry(0, DICT_LENGTH, &String::from_utf8_lossy(&tmp));
            println!("{}", loc);
        }
    }

    let output = Command::new("say")
        .arg("-v")
        .arg("sin-ji")
        .arg(&args[1])
        .spawn();
    assert!(output.is_ok());
}
