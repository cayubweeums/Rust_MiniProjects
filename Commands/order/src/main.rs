// Works like cat, except the output lines must be sorted before being output. All lines from all files will be mixed together and then sorted.
// If the “-r” command-line argument is provided, they should be sorted in reverse order.

use std::fs::File;
use std::io::{Result, BufReader, BufRead};
use std::env;
use std::env::args;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut flag = false;
    if args[1].contains("-r"){
        &args.remove(1);
        flag = true;
    }
    let mut all_content:  Vec<String> = vec![];
    for arg in args.iter().skip(1){

        let mut file = File::open(arg).expect("Error reading given file");
        let reader = BufReader::new(&mut file);
        let mut content: Vec<_> = reader.lines().map(|l| l.expect("could not read file")).collect();

        all_content.append(&mut content);
    }
    if flag{
        all_content.sort();
        all_content.reverse();
        for line in all_content{
            println!("{:?}", line);
        }
    }else{
        all_content.sort();
        for line in all_content{
            println!("{:?}", line);
        }
    }
}
