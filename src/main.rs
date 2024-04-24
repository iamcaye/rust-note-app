use std::io;
mod menu;

fn main() {
    loop {
        let mut question = String::new();
        menu::print();
        io::stdin().read_line(&mut question).unwrap();
        question = question.trim().to_string();
        menu::handle_input(&question);
    }
}
