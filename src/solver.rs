use std::{io::{stdin, BufReader, BufRead}, fs::File};
use inquire::Select;

pub fn update_known(inp: &mut String) -> Result<(), ()> {
    println!("Current knowns: {inp}");
    print!("Please type what the known characters are. Denote unknown characters with '*'.\n>> ");

    let mut new = String::new();
    stdin().read_line(&mut new).unwrap();
    new = new.trim().to_owned();

    if new.len() > 5 {
        new = new.as_str()[0..=4].to_owned();
    }
    while new.len () < 5 {
        new.push('*');
    }

    new = new.to_lowercase();
    println!("Is this what you want: \"{new}\"?");
    let m_items = ["YES".to_owned(), "NO".to_owned()];

    match Select::new("CONFIRM", m_items.to_vec())
    .prompt()
    .unwrap().as_str() {
        "YES" => *inp = new.clone(),
        _ => return Err(()),
    };

    println!();

    Ok(())
}

pub fn update_unknown_nons(inp: &mut String, non: bool) -> Result<(), ()> {
    let kind: String;
    if non {
        kind = "nons".to_owned();
    } else {
        kind = "unknowns".to_owned();
    }
    println!("Current {}: {inp}", kind);
    print!("Please type any letters to append. Prefix letters being removed with '-'.\n>> ");

    let mut new = String::new();
    stdin().read_line(&mut new).unwrap();
    new = new.trim().to_owned();

    let mut rm = false;
    let mut rms = String::new();
    let mut con = inp.clone();
    for c in new.chars() {
        if c == '-' {
            rm = true;
            continue;
        }
        else if rm {
            rms.push(c);
            continue;
        }
        else {
            con.push(c);
        }
    }

    rms = rms.to_lowercase();
    rms.chars().for_each(|c| con.remove_matches(c));
    con = con.to_lowercase();

    println!("Is this what you want: \"{con}\"?");
    let m_items = ["YES".to_owned(), "NO".to_owned()];

    match Select::new("CONFIRM", m_items.to_vec())
    .prompt()
    .unwrap().as_str() {
        "YES" => *inp = con.clone(),
        _ => return Err(()),
    };

    println!();

    Ok(())
}

pub fn solve(known: &String, unknown: &String, nons: &String) -> Option<Vec<String>> {
    let file = match File::open("list.txt") {
        Ok(e) => e,
        Err(_) => {
            println!("\nRequisite resource file 'list.txt' not found\n");
            return None;
        }
    };

    let mut list = BufReader::new(file)
    .lines()
    .map(|s| s.unwrap().trim().to_owned())
    .collect::<Vec<String>>();
    
    //Filter lines that contain letters known not to be in the result
    let mut filtered_list = Vec::<String>::new();
    'outer: for line in &list {
        for c in nons.chars() {
            if line.contains(c) {
                continue 'outer;
            }
        }
        filtered_list.push(line.to_string());
    }

    //Filter lines that do not contain letters known to be in the result
    list = filtered_list.clone();
    filtered_list.clear();
    'outer: for line in &list {
        for c in unknown.chars() {
            if !line.contains(c) {
                continue 'outer;
            }
        }
        filtered_list.push(line.to_string());
    }

    //Filter lines with the exact pattern given in the knowns
    list = filtered_list.clone();
    filtered_list.clear();
    'outer: for line in &list {
        let mut idx = 0;
        for c in known.chars() {
            if c != '*'
                && c != line.chars().nth(idx).unwrap() {
                    continue 'outer;
            }
            idx += 1;
        }
        filtered_list.push(line.to_string());
    }

    Some(filtered_list)
}