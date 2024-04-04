use std::collections::{HashMap, HashSet};
use std::io::{stdin, Write};

use regex::Regex;

enum State {
    Wait,
    Analyze,
    ChooseReplace,
    Replace,
}

pub(crate) fn substitution_runner() {
    println!("Enter your input string");
    let mut input = String::new();
    let mut modified = String::new();
    let mut map = HashMap::new();
    let mut state = State::Wait;
    let mut target = 'a';
    let mut get_in;

    loop {
        let _ = std::io::stdout().flush();
        get_in = stdin().read_line(&mut input);
        match state {
            State::Wait => {
                if get_in.is_ok() {
                    println!("Chosen String: {} press enter to continue", input);
                    state = State::Analyze;
                    modified = input.clone();
                }
            }
            State::Analyze => {
                map = analyze(modified.clone());
                state = State::Analyze;
                let mut set = HashSet::new();
                let mut row1 = Vec::new();
                let mut row2 = Vec::new();
                while set.len() != map.len() {
                    let mut curr = ('a', 0);
                    for (c, n) in &map {
                        if !set.contains(c) && n > &curr.1 {
                            curr = (*c, *n);
                        }
                    }
                    println!("{}, {}", curr.0, curr.1);
                    set.insert(curr.0);
                }
                println!("What letter to replace?");
                state = State::ChooseReplace;
            }
            State::ChooseReplace => {
                if let Some(c) = input.chars().nth(0) {
                    target = c;
                }
                println!("Replace with?");
                state = State::Replace;
            }
            State::Replace => {
                if let Some(c) = input.chars().nth(0) {
                    if let Ok(regex) = Regex::new(String::from(c).as_str()) {
                        if regex.find(modified.as_str()).is_some() {
                            modified = regex.replace_all(modified.as_str(), "~").to_string();
                            let regex = Regex::new(String::from(target).as_str());
                            if let Ok(regex) = regex {
                                modified = regex.replace_all(modified.as_str(), String::from(c)).to_string();
                            }
                            let regex = Regex::new("~").unwrap();
                            modified = regex.replace_all(modified.as_str(), String::from(target)).to_string();
                            println!("Swapped {} with {}", target, c);
                        } else {
                            let regex = Regex::new(String::from(target).as_str());
                            if let Ok(regex) = regex {
                                modified = regex.replace_all(modified.as_str(), String::from(c)).to_string();
                            }
                        }
                    }
                }
                println!("String is now: {} press enter to continue", modified);
                state = State::Analyze;
            }
        }
        input = String::from("");
    }
}

fn analyze(input: String) -> HashMap<char, u32> {
    let mut map: HashMap<char, u32> = HashMap::new();
    for char in input.chars() {
        if !char.is_ascii_alphanumeric() { continue; }
        let res = map.get(&char);
        match res {
            Some(mut num) => {
                map.insert(char, *num + 1);
            }

            None => {
                map.insert(char, 1);
            }
        }
    }
    map
}
