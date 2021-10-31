use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

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

pub fn display() {
    println!("{:?}", env::current_dir());
    let file_contents = open_file("notes.txt".to_string());
    println!("{}", file_contents);
    let data = parse_contents(&file_contents);
    println!("{:?}", data);
}

fn open_file(name: String) -> String {
    let mut contents = String::new();
    let file = File::open(name);
    match file {
        Ok(f) => {
            let mut buf_reader = BufReader::new(f);
            buf_reader.read_to_string(&mut contents);
            return contents;
        }
        Err(_) => "File not found.".to_string(),
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
