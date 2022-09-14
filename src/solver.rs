use std::io::stdin;
use inquire::Select;

pub fn update_known(inp: &mut String) -> Result<(), ()> {
    println!("Current knowns: {inp}");
    print!("Please type what the known characters are. Denote unknown characters with '*'.\n>> ");

    let mut new = String::new();
    stdin().read_line(&mut new).unwrap();
    new = new.trim().to_owned();

    println!("Is this what you want: \"{new}\"?");
    let m_items = ["YES".to_owned(), "NO".to_owned()];

    match Select::new("CONFIRM", m_items.to_vec())
    .prompt()
    .unwrap().as_str() {
        "YES" => *inp = new.clone(),
        "NO" => return Err(()),
        _ => return Err(()),
    };

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

    rms.chars().for_each(|c| con.remove_matches(c));

    println!("Is this what you want: \"{con}\"?");
    let m_items = ["YES".to_owned(), "NO".to_owned()];

    match Select::new("CONFIRM", m_items.to_vec())
    .prompt()
    .unwrap().as_str() {
        "YES" => *inp = con.clone(),
        "NO" => return Err(()),
        _ => return Err(()),
    };

    Ok(())
}

pub fn solve(known: &String, unknown: &String, nons: &String) -> String {
    todo!();
}