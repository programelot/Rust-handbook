// Panic will rewind the program
// However, program can just be aborted when it causes panic.
// To activate it, add follwoing to cargo.toml
//
// [profile.release]
// panic = 'abort'

// To see the back trace of panic,
// use following commands on powershell.
// $env:RUST_BACKTRACE=1
// $env:RUST_BACKTRACE="full"

// It can be deactivated by following command.
// $env:RUST_BACKTRACE=1

use std::fs::{self, File}; //Add file functions
use std::io::{self, ErrorKind, Read}; //Add error kinds, io functions

fn try_to_read_line() -> String {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Not able to read line");
    return s;
}

fn main() {
    loop {
        println!("p : Panic");
        println!("i : Read index");
        println!("o1 : Read file with match panic handling");
        println!("o2 : Read file with unwrap");
        println!("o3 : Read file with expect");
        println!("f1 : Read file in function 1");
        println!("f2 : Read file in function 2");
        println!("f3 : Read file in function 3");
        println!("f4 : Read file in function 4");
        println!("q : Quit");
        let mut s = try_to_read_line();
        let s = s.trim();
        if s == "q" {
            break;
        } else if s == "p" {
            //Manual panic
            panic!("Panic!");
        } else if s == "i" {
            //Index overflow.
            let v = [1, 2, 3, 4, 5];
            println!("Array defined as follow");
            for e in v {
                print!("{e} ");
            }
            println!("");
            let mut s = try_to_read_line();
            let i: usize = s.trim().parse().expect("Unable to parse number");
            println!("index {0} value is {1}", i, v[i]);
        } else if s == "o1" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            let f = File::open(&s);
            let f = match f {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    //error kind can be known by error.kind()
                    ErrorKind::NotFound => match File::create(&s) {
                        Ok(fc) => fc,
                        Err(e) => panic!("File can not be created due to {:?}", e),
                    },
                    other => panic!("File read failed due to {:?}", error),
                },
            };
        } else if s == "o2" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            let f = File::open(&s).unwrap(); // It will be in panic if it is not Ok.
        } else if s == "o3" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            let f = File::open(&s).expect("Failed to open file"); // Programmer can set error message when using expect.
        } else if s == "f1" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            //It is the same with f3 but got error passed from function.
            let r = read_file1(&s).expect("Failed to open file");
            println!("Got string in {0} : {1}", s, r);
        } else if s == "f2" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            //It is the same with f3 but got error passed from function.
            let r = read_file2(&s).expect("Failed to open file");
            println!("Got string in {0} : {1}", s, r);
        } else if s == "f3" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            //It is the same with f3 but got error passed from function.
            let r = read_file3(&s).expect("Failed to open file");
            println!("Got string in {0} : {1}", s, r);
        } else if s == "f4" {
            //File error
            println!("Type the file name");
            let mut s = try_to_read_line();
            let s = s.trim();
            //It is the same with f3 but got error passed from function.
            let r = read_file4(&s).expect("Failed to open file");
            println!("Got string in {0} : {1}", s, r);
        }
    }
}

fn read_file1(file_name: &str) -> Result<String, io::Error> {
    let file = File::open(file_name);
    let mut file_unwrap = match file {
        Ok(file) => file,
        Err(e) => return Err(e), //If file not exists, return error.
    };
    let mut s = String::new();
    match file_unwrap.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// read_file2 works the same with read_file1.
// Question mark(?) at the end of the expresseion is a syntatic sugar.
// It unwrap when it is okay, otherwise return error.
// The only difference is that ? goes through from function.
// ** CAUTION **
// To use ? operator, return type must be Result or Option.
fn read_file2(file_name: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_name)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

// It can be even shorter.
fn read_file3(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}

//Using fs can make it even shorter.
fn read_file4(file_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(file_name)
}
