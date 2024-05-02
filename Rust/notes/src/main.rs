//to run: cargo-watch -qc -x "run -- 'YOUR NOTE'" -i "notes.txt" -x clippy
//to run : cargo run -- "Your notes"
//-i option stands for "ignore." This tells cargo-watch to ignore changes in the specified file(s) or path. Here, it's set to ignore any changes in "notes.txt"

#![deny(clippy::all)]
//Working with reading application argumants we need std::env
use std::env;
//open options: very usefull in files allow us to append to, read from and write to files
use std::fs::OpenOptions;
//to be able to work with files
// * : import everything
use std::io::prelude::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //collect arg
    let args: Vec<String> = env::args().collect();

    //less than 2 args has pass only
    if args.len() != 2 {
        println!("Usage: notes your_note_goes_here");
        // A status code of 1 generally indicates an error. This is a typical way to stop a program when an invalid state or error condition is encountered.
        //std::process::exit(1);
    }

    //keyword (arg)
    // let note = &args[1];
    //OR
    let note = args[1].clone();

    //garb the current date and time
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:&S").to_string();

    //create file to write and append
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    //take contents of now variable and store inside this file
    //write the current date and type
    //html comment
    file.write_all(b"<!--")?; //begin to store partial string
    file.write_all(now.as_bytes())?; //take date and time , store it as bytes
    file.write_all(b"-->\n")?;

    //store actual note into the file
    file.write_all(note.as_bytes())?;
    file.write_all(b"\n\n")?;
    Ok(())
}
