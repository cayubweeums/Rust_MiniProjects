// start: Prints out the first ten lines of each file listed in the command-line arguments. If the first argument begins with a dash, use the number immediately following the dash instead of ten.

use std::env::{args, Args};
use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::env;
use std::ptr::null;

fn main() -> std::io::Result<()>{
    let mut num = 10;

    for arg in args().skip(1){
        if arg.contains("-"){
            println!("{:?}", arg);
            let pre_num = arg.replace('-', "");
            num = pre_num.parse().unwrap();
            println!("{:?}", num);
        }else {
            let file = File::open(arg)?;
            let reader = BufReader::new(file);
            for (index, line) in reader.lines().enumerate(){
                while index < num{
                    println!("{}. {:?}", index + 1, &line.unwrap());
                    break
                }
            }
        }
    }
    Ok(())
}
