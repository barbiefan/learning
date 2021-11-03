use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::process;

// this is actually so bad I can't even...
// I just want to practice writing some shit in rust
// Coulde've been easily implemented via separate files and dedicated folder.
// But here I am. Inventing a wheel that will roll down the hill and bust my nuts off.
// 1st. this shouldn't be implemented via single file. There's just no need for that.
// 2nd. the way I'm iterating through the contents of the file is ridiculous.
// 3rd. Just look at the END_OF_NOTE. This is so insecure... End me.

const END_OF_NOTE: &str = "\n---end@of@note---\n"; // lmao
const END_OF_SECTION: &str = "\n---end@of@section---\n"; // lmao

#[derive(Debug)]
struct Note {
    name: String,
    date: i32,
    text: String,
}

fn help() {
    println!("'new': create new note\n'list': list all notes\n'quit': exit app")
}

fn list(data: Vec<Note>) {
    println!("{:?}", data)
}

pub fn main_loop() {
    println!("type a file name to open (empty for notes.txt): ");
    let file_name = get_input();
    let file_name = match &file_name[..] {
        "\n" => "notes.txt".to_string(),
        &_ => file_name,
    };
    loop {
        let file_contents = read_file(&file_name);
        let data = parse_contents(&file_contents);
        println!("command (h for help): ");
        match &get_input()[..] {
            "h\n" => {help()},
            "quit\n" => {process::exit(0)},
            "help\n" => {help()},
            //"new\n" => {new()},
            "list\n" => {list(data)},
            &_ => println!("Unknown command. Try typing h or help.")
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");
    return input
}

fn read_file(name: &String) -> String {
    let mut contents = String::new();
    let file = File::open(name);
    match file {
        Ok(f) => {
            let mut buf_reader = BufReader::new(f);
            // should handle this...
            buf_reader.read_to_string(&mut contents);
            return contents;
        }
        Err(_) => "File not found.".to_string(),
    }
}

fn write_file(name: &String, contents: &String) -> String {
    let mut file = match File::create(&name) {
        Err(why) => return format!("Error: {}", why),
        Ok(file) => file,
    };
    match file.write_all(contents.as_bytes()) {
        Err(why) => format!("Error: {}", why),
        Ok(_) => format!("Successfuly wrote to {}", name),
    }
}

fn parse_contents(contents: &String) -> Vec<Note> {
    let mut data: Vec<Note> = Vec::new();
    let split = contents.split(END_OF_NOTE);
    for note in split {
        let mut sections = note.split(END_OF_SECTION);
        let note_structured = Note {
            name: match sections.next() {
                Some(val) => val.to_string(),
                None => "Couldn't load note Name".to_string(),
            },
            date: match sections.next() {
                Some(val) => match val.parse::<i32>() {
                    Ok(integer32) => integer32,
                    Err(_) => 0,
                },
                None => 0,
            },
            text: match sections.next() {
                Some(val) => val.to_string(),
                None => "Couldn't load note text".to_string(),
            },
        };
        data.push(note_structured);
    }
    return data;
}
