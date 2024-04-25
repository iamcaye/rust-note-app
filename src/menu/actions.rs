use chrono;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn add_note() {
    println!("Add a note");
    let new_note: super::Note = super::Note {
        id: unsafe { super::NOTES.len() as i32 },
        title: get_input("Title: "),
        content: get_input("Content: "),
        created_at: chrono::Local::now().to_string(),
    };
    println!("Note added: {:?}", new_note);
    unsafe {
        super::NOTES.push(new_note);
    }
}

pub fn list_notes() {
    println!("List all notes");
    unsafe {
        for note in &super::NOTES {
            println!("{:?}", note);
        }
    }
}

pub fn remove_note() {
    println!("Remove a note");
    let id = get_input("Enter the ID of the note to remove: ");
    unsafe {
        let id = id.parse::<i32>().unwrap();
        let index = super::NOTES.iter().position(|note| note.id == id);
        match index {
            Some(index) => {
                super::NOTES.remove(index);
                println!("Note removed");
            }
            None => println!("Note not found"),
        }
    }
}

pub fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}
