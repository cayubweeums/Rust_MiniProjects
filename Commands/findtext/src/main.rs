// Output every line that contains a specified pattern. The first command-line argument is the fixed-string pattern.
// Remaining arguments are the names of the files to inspect.


use std::io::{self, Write};
use std::env::args;


fn main()  {
    let pattern = std::env::args().nth(1).expect("No pattern given ### 1 ###");
    let mut paths = <Vec<String>>::new();
    for parsing in args().skip(2){
        paths.push(parsing);
    }
    for path in paths{
        let file_strings = std::fs::read_to_string(&path)
            .expect("could not read file");
        for line in file_strings.lines(){
            if line.contains(&pattern){
                println!("{:?}", line);
            }
        }
    }
}



// https://rust-cli.github.io/book/tutorial/cli-args.html