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
        "CLEAR" => CLEAR,
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
    CLEAR,
    EXIT
}

fn main() {
    let m_items = ["KNOWNS".to_owned(),
    "UNKNOWNS".to_owned(),
    "NONS".to_owned(),
    "SOLVE".to_owned(),
    "CLEAR".to_owned(),
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
                        println!("The new knowns: \"{}\"\n", known);
                    },
                    Err(_) => {
                        println!("The knowns are still \"{}\"\n", known);
                    },
                };
            },
            State::UNKNOWN => {
                match solver::update_unknown_nons(&mut unknown, false) {
                    Ok(_) => {
                        println!("The new unknowns: \"{}\"\n", unknown);
                    }
                    Err(_) => {
                        println!("THe unknowns are still \"{}\"\n", unknown);
                    }
                };
            },
            State::NON => {
                match solver::update_unknown_nons(&mut non, true) {
                    Ok(_) => {
                        println!("The new nons are: \"{}\"\n", non);
                    }
                    Err(_) => {
                        println!("The nons are still \"{}\"\n", non);
                    }
                };
            },
            State::CLEAR => {
                known = "*****".to_owned();
                unknown = String::new();
                non = String::new();
                println!("Stored Values Cleared!\n");
            }
            State::SOLVING => {
                let result = solver::solve(&known, &unknown, &non);
                if let Some(e) = result {
                    if e.len() > 0 {
                        println!("Results:");
                        e.iter().for_each(|s| println!("   > {s}"));
                        println!("There are {} possible answers\n", e.len());
                    } else {
                        println!("No Results!\n");
                    }
                }
            },
            _ => break,
        };
        state = menu(&m_items);
        println!();
    }
}
