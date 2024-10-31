//#![feature(char_indices_offset)]
//use std::str::CharIndices;
use std::env;
use std::process::Command;
use std::io::stdin;
use std::io::Read;

mod constants;
use constants::DICT_LENGTH;
use constants::DICT;

enum AudioLanguageOption {
    None,
    Cantonese,
    Mandarin,
}

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

fn get_info(phrase: &str, audio_language: AudioLanguageOption) {
    let char_indices: Vec<(usize, char)> = phrase.char_indices().collect();
    for idx in 0 .. char_indices.len() - 1 {
        let curr: (usize, char) = *char_indices.get(idx).unwrap();
        let offset: usize = char_indices.get(idx + 1).unwrap().0;
        let loc = find_entry(0, DICT_LENGTH, &(phrase)[curr.0 .. offset]);

        if loc < DICT_LENGTH {
            println!("{hanzi:<7}{pinyin:<7}{jyutping:<7}", hanzi=DICT[loc][0], pinyin=DICT[loc][2], jyutping=DICT[loc][1]);
        } else if curr.1 == '\n' {
            println!("");
        }
        else if curr.1 >= 32 as char {
            println!("{}", curr.1);
        }
    }
    let curr: (usize, char) = *char_indices.get(char_indices.len() - 1).unwrap();
    let loc = find_entry(0, DICT_LENGTH, &(phrase)[curr.0 .. phrase.len()]);
    if loc < DICT_LENGTH {
        println!("{hanzi:<7}{pinyin:<7}{jyutping:<7}", hanzi=DICT[loc][0], pinyin=DICT[loc][2], jyutping=DICT[loc][1]);
    } else if curr.1 == '\n' {
        println!("");
    } else if curr.1 >= 32 as char {
        println!("{}", curr.1);
    }

    // This code is much cleaner, but requires: #![feature(char_indices_offset)]
    //let char_itr: CharIndices = args[1].char_indices();
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
        match audio_language {
            AudioLanguageOption::None => { }
            AudioLanguageOption::Cantonese => {
                let result = Command::new("say")
                .arg("-v")
                // .arg("Sinji (Premium)")
                .arg("Sinji")
                .arg(&phrase)
                .spawn();
                assert!(result.is_ok());
            }
            AudioLanguageOption::Mandarin => {
                let result = Command::new("say")
                .arg("-v")
                // .arg("Tingting (Enhanced)")
                .arg("Tingting")
                .arg(&phrase)
                .spawn();
                assert!(result.is_ok());
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut buff: Vec<u8> = Vec::new();
        println!("[return + CTRL + D] to finish input, [CTRL + D] to exit.");
        println!("Input:");
        
        while let Ok(n) = stdin().read_to_end(&mut buff) {
            if n == 0 { break; }
            let input: String = std::str::from_utf8(&buff)
                .expect("buff to str failed.")
                .to_string();
            
            get_info(&input, AudioLanguageOption::None);

            println!("[return + CTRL + D] to finish input, [CTRL + D] to exit.");
            println!("Input:");
            buff.clear();
        }
    } else if args.len() == 2 {
        get_info(&args[1], AudioLanguageOption::None);
    } else if args.len() == 3 {
        if args[2] == "--cantonese" || args[2] == "-c" || args[2] == "c" {
            get_info(&args[1], AudioLanguageOption::Cantonese);
        }
        else if args[2] == "--mandarin" || args[2] == "-m" || args[2] == "m" {
            get_info(&args[1], AudioLanguageOption::Mandarin);
        }
    }
}
