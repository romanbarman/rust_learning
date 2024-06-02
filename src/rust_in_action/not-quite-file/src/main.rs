#![allow(dead_code)]
use std::fmt;
use std::fmt::{Display, Formatter, write};

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Close
}

impl Display for FileState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Close => write!(f, "CLOSED")
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Close
        }
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Close;
    Ok(f)
}

fn main() {
    let file_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    
    
    let mut file = File::new_with_data("file.txt", &file_data);
    
    let mut buffer: Vec<u8> = vec![];
    
    if file.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }
    
    file = open(file).unwrap();
    let f1_length = file.read(&mut buffer).unwrap();
    file = close(file).unwrap();
    
    let text = String::from_utf8_lossy(&buffer);
    
    println!("{:?}", file);
    println!("{}", file);
    println!("{} is {} bytes long", &file.name, f1_length);
    println!("{}", text);
}
