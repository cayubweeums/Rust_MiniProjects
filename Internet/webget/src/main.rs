use std::{io, env};
use std::net::TcpStream;
use std::io::{Write, Read};
use openssl::ssl::{SslConnector, SslMethod};
use std::fs::File;


struct CommandParameters{
    hostname: String,
    file: String,
    https: bool,
    vec_commands: Vec<String>,
    port: String
}

fn main() -> io::Result<()>{

    let user_commands = get_input("webget ")?;
    if user_commands.len() > 0 {
        let command_parameters = CommandParameters{
            hostname: "".to_string(),
            file: "".to_string(),
            https: false,
            vec_commands: user_commands.split("://").map(|x| x.to_string()).collect(),
            port: "".to_string()
        };
        let parsed_commands = get_command_parameters(command_parameters);
        let message = create_message(parsed_commands.hostname.clone(), parsed_commands.file.clone());
        send_message(&*parsed_commands.hostname, parsed_commands.port.parse().unwrap(), &*message, &*parsed_commands.file);
    }
    Ok(())
}


fn get_command_parameters(mut commands: CommandParameters) -> CommandParameters{
    let item = commands.vec_commands.clone();

    let mut temp_vec = item[1].splitn(2, "/");
    commands.vec_commands.drain(1..2);

    let x = temp_vec.next().unwrap();
    let y = temp_vec.next().unwrap();
    commands.vec_commands.push(x.to_string());
    commands.vec_commands.push(format!("{}{}", "/", y.to_string()));

    if commands.vec_commands[0].contains("https") {
        commands.https = true;
        commands.vec_commands.remove(0);
    }else {
        commands.vec_commands.remove(0);
    }
    if !commands.vec_commands[0].contains(":") {
        commands.hostname = commands.vec_commands[0].clone();
        let https = "443";
        let http = "80";
        if commands.https {
            commands.port = https.to_string();
        }else {
            commands.port = http.to_string();
        }
    }else {
        let port_vec: Vec<String> = commands.vec_commands[0].clone().split(":").map(|x| x.to_string()).collect();
        commands.hostname = port_vec[0].clone();
        commands.port = port_vec[1].clone();
    }
    commands.file = commands.vec_commands[1].clone();
    return commands;
}

fn create_message(host: String, file: String) -> String{
    let message =  "GET ".to_owned() + &*file + " HTTP/1.1\n" + "Host: " + &*host + "\nConnection: Close \n\n";
    return message
}

fn send_message(host: &str, port: usize, message: &str, filename: &str) -> io::Result<()> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let connector = SslConnector::builder(SslMethod::tls())?.build();
    let mut stream = connector.connect(host, tcp).unwrap();
    stream.write(message.as_bytes())?;
    let mut reply = String::new();
    stream.read_to_string(&mut reply)?;
    let new_line: Vec<String> = reply.split("\r\n\r\n").map(|s| s.to_string()).collect();
    let final_lines: Vec<String> = new_line[0].split("\r\n").map(|s| s.to_string()).collect();
    for line in final_lines{
        println!("{:?}", line);
    }
    let temp_file: Vec<String> = filename.split("/").map(|s| s.to_string()).collect();
    let path = &temp_file[temp_file.len() -1];
    let mut file = File::create(path)?;
    write!(file, "{:?}",new_line[1]);
    Ok(())
}


fn get_input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{} ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    buffer.pop();
    Ok(buffer)
}



// https://stackoverflow.com/questions/41517187/split-string-only-once-in-rust
