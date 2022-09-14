mod solver;
use inquire::Select;

fn menu(items: &[String]) -> State {
    use State::*;
    match Select::new("MENU", items.to_vec())
    .prompt()
    .unwrap().as_str() {
        "Update Known Locations" => KNOWN,
        "Update Unknown Characters" => UNKNOWN,
        "Update Known Non-Characters" => NON,
        "SOLVE" => SOLVING,
        "EXIT" => EXIT,
        _ => EXIT,
    }
}

#[derive(PartialEq, Eq)]
enum State {
    READY,
    KNOWN,
    UNKNOWN,
    NON,
    SOLVING,
    EXIT
}

fn main() {
    let m_items = ["Update Known Locations".to_owned(), 
    "Update Unknown Characters".to_owned(), 
    "Update Known Non-Characters".to_owned(), 
    "SOLVE".to_owned(), 
    "EXIT".to_owned()];

    let mut known = "*****".to_owned();
    let mut state = menu(&m_items);
    loop {
        match state {
            State::KNOWN => {
                solver::update_known(&mut known).unwrap();
                println!("The new knowns: {}", known);
            },
            State::UNKNOWN => {},
            State::NON => {},
            State::SOLVING => {},
            _ => break,
        };
        state = menu(&m_items);
    }
}
