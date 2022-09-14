#![feature(string_remove_matches)]

mod solver;
use inquire::Select;

fn menu(items: &[String]) -> State {
    use State::*;
    match Select::new("MENU", items.to_vec())
    .prompt()
    .unwrap().as_str() {
        "KNOWNS" => KNOWN,
        "UNKNOWNS" => UNKNOWN,
        "NONS" => NON,
        "SOLVE" => SOLVING,
        "EXIT" => EXIT,
        _ => EXIT,
    }
}

#[derive(PartialEq, Eq)]
enum State {
    KNOWN,
    UNKNOWN,
    NON,
    SOLVING,
    EXIT
}

fn main() {
    let m_items = ["KNOWNS".to_owned(), 
    "UNKNOWNS".to_owned(), 
    "NONS".to_owned(), 
    "SOLVE".to_owned(), 
    "EXIT".to_owned()];

    let mut known = "*****".to_owned();
    let mut unknown = String::new();
    let mut non = String::new();
    let mut state = menu(&m_items);
    loop {
        match state {
            State::KNOWN => {
                match solver::update_known(&mut known) {
                    Ok(_) => {
                        println!("The new knowns: \"{}\"", known);
                    },
                    Err(_) => {
                        println!("The knowns are still \"{}\"", known);
                    },
                };
            },
            State::UNKNOWN => {
                match solver::update_unknown_nons(&mut unknown, false) {
                    Ok(_) => {
                        println!("The new unknowns: \"{}\"", unknown);
                    }
                    Err(_) => {
                        println!("THe unknowns are still \"{}\"", unknown);
                    }
                };
            },
            State::NON => {
                match solver::update_unknown_nons(&mut non, true) {
                    Ok(_) => {
                        println!("The new nons are: \"{}\"", non);
                    }
                    Err(_) => {
                        println!("The nons are still \"{}\"", non);
                    }
                };
            },
            State::SOLVING => {},
            _ => break,
        };
        state = menu(&m_items);
    }
}
