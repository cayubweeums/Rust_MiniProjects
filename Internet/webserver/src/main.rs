use std::net::{TcpListener, TcpStream};
use std::{thread, fs, env};
use std::io;
use std::io::prelude::*;
use std::fs::{File, read_dir};
use std::thread::JoinHandle;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::borrow::{Cow, Borrow};
use std::sync::{Arc, Mutex};


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8888").unwrap();
    let mut incoming = listener.incoming();

    let mut handles = vec![];
    let total_requests = Arc::new(Mutex::new(0));
    let valid_requests = Arc::new(Mutex::new(0));

    while let Some(stream) = incoming.next(){
        let total_requests = Arc::clone(&total_requests);
        let valid_requests = Arc::clone(&valid_requests);
        println!("Total Requests: {}\nValid Requests: {}", total_requests.lock().unwrap(), valid_requests.lock().unwrap());
        let stream = stream?;
        let handle = thread::spawn(move || {
            let mut tot = total_requests.lock().unwrap();
            // let mut val = valid_requests.lock().unwrap();
            handle_client(stream);
            *tot += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}


fn handle_client(mut stream: TcpStream) -> std::io::Result<()>{
    let mut buffer = [0; 500];
    stream.read(&mut buffer).unwrap();

    let temp_str = String::from_utf8_lossy(&buffer[..]);
    let temp_vec: Vec<&str> = temp_str.split(|c| c == '/' || c == ' ' || c == '\r').filter(|s| !s.is_empty()).collect();
    let filename = &*temp_vec[1].to_string();
    println!("Requested File: {:?}#####", filename);


    let mut txt_files = vec![];
    let path = env::current_dir()?;
    for entry in fs::read_dir(path)? {
        let entry = entry?.path();
        if let Some("txt") = entry.extension().and_then(OsStr::to_str){
            txt_files.push(entry);
        }
    }


    let mut response = String::new();


    response = create_response(filename, txt_files);

    stream.write(response.as_ref()).unwrap();
    stream.flush().unwrap();

    Ok(())
}


fn create_response(filename: &str, txt_files: Vec<PathBuf>) -> String{
    let mut response = String::new();
    for i in txt_files{
        if i.file_stem().unwrap() == filename{
            let contents = fs::read_to_string(i);
            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: \r\n\r\n<html>\r\n<body>\r\n<h1>Requested File Contents: </h1>\r\n{}<br>\r\n</body>\r\n</html>\r\n", contents.unwrap());
            break
        }else {
            response = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Length: \r\n\r\n");
        }
    }
    return response;
}

