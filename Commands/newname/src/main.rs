// newname: This program expects two command-line arguments. It will give a “usage” message if it does not receive them. It will change the name of the file given by the first argument to be the name given by the second argument.

use std::env;
use std::fs;


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    match args.len(){
        1 => {
            println!("Not enough arguments received. Need -OldFileName- & -NewFileName- to run command");
        }

        3 => {
            let from = &args[1];
            let to = &args[2];
            fs::rename(from, to)?;
            println!("Renamed {:?} --> {:?}", from, to);
        }
        _ => {
            println!("No valid arguments received. Need only -OldFileName- & -NewFileName- to run command");
        }
    }
    Ok(())
}
