// If the line ends with the & symbol, it should run in the background. That is, your shell should not wait for it to terminate; the command line should immediately return. Your shell should print the PID of the process, so that the user may later manage it as needed. This is typically used for long-running programs that perform a lot of computation. It is most often used in conjunction with output redirection, as described in step 3.
// The line may contain two or more commands connected with the pipe symbol (|). If this happens, start a process for each command, setting up pipes to send the output of each left-hand command to the input of the following right-hand command.
// The last command in the pipeline (or the only command, if there is no pipeline) may be followed by the > symbol and a filename. The command’s output should be stored in the designated file. If the file does not exist, it should be created.
// The first command in the pipeline (or the only command, if there is no pipeline) may be followed by the < symbol and a filename. The command’s input should be taken from the designated file. If the file does not exist, the command should abort.

use nix::unistd::{fork, ForkResult, execvp, pipe, close, dup2};
use std::{io, env};
use std::ffi::CString;
use nix::sys::wait::waitpid;
use std::io::Write;
use std::ops::Index;


struct CommandParameters{
    background: bool,
    output_file: String,
    input_file: String,
    pipelined_commands: Vec<String>
}

fn main() -> io::Result<()>{
    loop{
        let path = env::current_dir()?;
        println!("{:?}", path);
        let user_command = get_input("Type Command Here => ")?;
        if user_command.len() > 0 {
            let externalized = externalize(&*user_command);
            if user_command == "exit" || user_command == "exit "{
                break;
            }
            if externalized[0].to_str() == Ok("cd"){
                let path = externalized[1].to_str();
                env::set_current_dir(path.unwrap());
            }else {
                let pipelined_commands: Vec<String> = user_command.split("|").map(|s| s.to_string()).collect();
                let mut command_parameters = CommandParameters{
                    background: false,
                    input_file: " ".to_string(),
                    output_file: " ".to_string(),
                    pipelined_commands: pipelined_commands.clone(),
                };
                let commands = input_handler(command_parameters);
                match unsafe {fork()}.unwrap(){
                    ForkResult::Parent {child} => {
                        waitpid(child, Option::None).unwrap();
                    }
                    ForkResult::Child => {
                        let (cmd_1, cmd_2) = pipe().unwrap();
                        match unsafe {fork()}.unwrap(){
                            ForkResult::Parent {child } => {
                                let exec = externalize(pipelined_commands[pipelined_commands.len() - 1].as_str());
                                dup2(cmd_1, 0).unwrap();
                                close(cmd_2).unwrap();
                                execvp(&exec[0], &*exec).unwrap();
                            }
                            ForkResult::Child => {
                                close(cmd_1).unwrap();
                                let (cmd_2_in, cmd_3) = pipe().unwrap();
                                match unsafe {fork()}.unwrap() {
                                    ForkResult::Parent {child} => {
                                        close(cmd_3).unwrap();
                                        dup2(cmd_2, 1).unwrap();
                                        dup2(cmd_2_in, 0).unwrap();
                                        let array = externalize(pipelined_commands[pipelined_commands.len() - 1].as_str());
                                        execvp(&array[0], &*array).unwrap();
                                    },
                                    ForkResult::Child => {
                                        close(cmd_2_in).unwrap();
                                        dup2(cmd_3, 1).unwrap();
                                        let array = externalize(pipelined_commands[0].as_str());
                                        execvp(&array[0], &*array).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn input_handler(mut commands: CommandParameters) -> CommandParameters {
    if commands.pipelined_commands[0].contains("<"){
        let temp: Vec<&str> = commands.pipelined_commands[0].split("<").collect();
        commands.input_file = temp[1].to_string();
    }
    if commands.pipelined_commands[commands.pipelined_commands.len() - 1].contains(">"){
        let temp: Vec<&str> = commands.pipelined_commands[commands.pipelined_commands.len() - 1].split(">").collect();
        commands.output_file = temp[0].to_string();
    }
    return commands;
}

fn externalize(command: &str) -> Box<[CString]> {
    let converted = command.split_whitespace()
        .map(|s| CString::new(s).unwrap())
        .collect::<Vec<_>>();
    converted.into_boxed_slice()
}


fn get_input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    print!("{} ", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    buffer.pop();
    Ok(buffer)
}


