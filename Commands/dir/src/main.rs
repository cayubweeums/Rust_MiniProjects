// dir: Prints out all of the names of the files in the current directory. It will not employ any command-line arguments
use std::{io,fs, env};
use std::path::Path;

fn main() -> std::io::Result<()>{
    let path = env::current_dir()?;
    if path.is_dir(){
        for entry in fs::read_dir(path)?{
            let entry = entry?;
            println!("{:?}", entry.file_name());
        }
    }else {
        println!("Path {} is not a dir", path.display());
    }
    Ok(())
}
