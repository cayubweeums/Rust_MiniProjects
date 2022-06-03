// duplicate: This program also expects two command-line arguments. It will give a “usage” message if it does not receive them. It will make a copy of the file given by the first argument with the name given by the second argument.

use std::env;
use std::fs;


fn main() -> std::io::Result<()>{
    let args: Vec<String> = env::args().collect();
    match args.len(){
        1 => {
            println!("Not enough arguments received. Need -FileToBeCopied- & -NewFileName- to run command");
        }

        3 => {
            let from = &args[1];
            let to = &args[2];
            fs::copy(from, to)?;
            println!("Copied the Contents of {:?} --> {:?}", from, to);
        }
        _ => {
            println!("No valid arguments received. Need only -FileToBeCopied- & -NewFileName- to run command");
        }
    }
    Ok(())
}
