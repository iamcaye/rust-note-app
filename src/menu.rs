use std::io::stdin;

mod actions;

#[derive(Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

pub enum Menu {
    Add,
    Remove,
    List,
    Exit,
    Invalid,
}

// local variable to store notes
pub type Notes = Vec<Note>;
pub static mut NOTES: Notes = Vec::new();

pub fn print() {
    println!("\x1B[2J\x1B[1;1H");
    println!("Menu:");
    println!("1. Add a note");
    println!("2. Remove a note");
    println!("3. List all notes");
    println!("4. Exit");
}

pub fn handle_input(input: &str) {
    handle_menu(match input {
        "1" => Menu::Add,
        "2" => Menu::Remove,
        "3" => Menu::List,
        "4" => Menu::Exit,
        _ => Menu::Invalid,
    })
}

fn handle_menu(menu: Menu) {
    match menu {
        Menu::Add => actions::add_note(),
        Menu::Remove => actions::remove_note(),
        Menu::List => actions::list_notes(),
        Menu::Exit => actions::exit(),
        Menu::Invalid => println!("Invalid text input"),
    }
    println!("Press Enter to continue...");
    let _ = stdin().read_line(&mut String::new());
}
