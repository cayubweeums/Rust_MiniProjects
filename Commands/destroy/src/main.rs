// destroy: Delete every file in the list of command-line arguments.
use std::env;
use std::fs;


fn main() -> std::io::Result<()>{
    for args in env::args().skip(1){
        fs::remove_file(args);
    }
    Ok(())
}
