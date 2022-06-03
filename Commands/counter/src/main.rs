// counter: Prints out the number of words, lines, and characters for each file listed in its command-line arguments.
// If the first argument begins with a dash, the letters “w”, “l”, and “c” immediately following the dash indicate which of words, lines, and characters get displayed.

use std::env::{args, Args};
use std::fs::File;
use std::io::{BufReader, BufRead, Result, Write};
use std::{env, io};
use std::ptr::null;

fn main() -> std::io::Result<()>{
    let mut flag = ' ';
    let mut curr_file = "";

    for arg in args().skip(1){
        if arg.contains("-"){
            flag = arg.replace('-', "").parse().unwrap();
        }match flag {
            'w' => {
                curr_file = &*arg;
                let mut words = 0;
                let file = File::open(curr_file)?;
                let reader = BufReader::new(file);
                for l in reader.lines(){
                    let curr_l = l.unwrap();
                    words += curr_l.split_whitespace().count();
                }
                println!("{:?}: total words = {:?}", curr_file, words)
            }
            'c' => {
                curr_file = &*arg;
                let mut chars = 0;
                let file = File::open(curr_file)?;
                let reader = BufReader::new(file);
                for l in reader.lines(){
                    let curr_l = l.unwrap();
                    chars = chars + curr_l.len() + 1;
                }
                println!("{:?}: total chars = {:?}", curr_file, chars)
            }
            'l' => {
                curr_file = &*arg;
                let mut lines = 0;
                let file = File::open(curr_file)?;
                let reader = BufReader::new(file);
                for l in reader.lines(){
                    lines += 1;
                }
                println!("{:?}: total lines = {:?}", curr_file, lines)

            }
            _ => {
                let mut words = 0;
                let mut chars = 0;
                let mut lines = 0;
                curr_file = &*arg;
                let file = File::open(curr_file)?;
                let reader = BufReader::new(file);
                for l in reader.lines(){
                    let curr_l = l.unwrap();
                    words += curr_l.split_whitespace().count();
                    chars = chars + curr_l.len() + 1;
                    lines = lines + 1;
                }
                println!("{:?}: \ntotal words = {:?}\n total chars = {:?}\n total lines = {:?}\n ", curr_file, words, chars, lines)
            }
        }

    }
    Ok(())
}
