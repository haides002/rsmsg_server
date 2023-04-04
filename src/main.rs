use std::{fs, path::Path};
use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

pub const FILENAME: &str = "chat";
pub const PORT: &str = "8000";
pub const SEPERATOR: &str = "\\(seperator)\\";

pub fn read_file(name: &str) -> String {
    fs::read_to_string(name).expect(&format!("Couldn't read {}", name))
}

pub fn append_to_file(file_name: &str, data: &str) {
    let mut file = if Path::exists(Path::new(&file_name)) {
        fs::OpenOptions::new().append(true).open(file_name).unwrap()
    } else {
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_name)
            .unwrap()
    };
    _ = file.write(data.as_bytes());
    _ = file.flush();
}

fn main() {
    let ip = format!("0.0.0.0:{PORT}");
    let listener = TcpListener::bind(ip).unwrap();
    for stream in listener.incoming() {
        _ = thread::spawn(|| {
            let mut stream = stream.unwrap();
            let mut buffer = vec![0; 65536];
            stream.read(&mut buffer).unwrap();

            let exclude_first: Vec<String> = String::from_utf8_lossy(&buffer[..])
                .split("\n\r\n")
                .map(|str| str.trim().trim_matches(char::from(0)).to_string())
                .collect();

            if exclude_first[0].contains(&"POST / HTTP/1.1".to_string()) {
                print!(">_< incomming post >_<\n");
                println!("{}", &exclude_first[1]);

                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();

                append_to_file(FILENAME, &format!("{}\n{}\n", SEPERATOR, exclude_first[1]));
            } else if exclude_first[0].contains(&"GET / HTTP/1.1".to_string()) {
                print!(">_< incomming get >_<\n");

                stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                stream.write_all(read_file(&FILENAME).as_bytes()).unwrap();
            } else {
                println!(">n< wrong request >n<");
                stream
                    .write_all(b"HTTP/1.1 405 Method Not Allowed")
                    .unwrap();
            }
        });
    }
}