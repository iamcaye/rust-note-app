pub enum Menu {
    Add,
    Remove,
    List,
    Exit,
}

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
        _ => {
            println!("Invalid input");
            Menu::Exit
        }
    })
}

fn handle_menu(menu: Menu) {
    match menu {
        Menu::Add => println!("Add a note"),
        Menu::Remove => println!("Remove a note"),
        Menu::List => println!("List all notes"),
        Menu::Exit => println!("Exit"),
    }
}
