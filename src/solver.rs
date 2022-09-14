use std::io::{stdin, stdout, Write};
use inquire::Select;

pub fn update_known(inp: &mut String) -> Result<(), ()> {
    println!("{inp}");
    print!("Please type what the known characters are. Denote unknown characters with '*'\n>> ");
    let mut new = String::new();
    stdin().read_line(&mut new).unwrap();
    new = new.trim().to_owned();
    println!("Is this what you want: {new}?");
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