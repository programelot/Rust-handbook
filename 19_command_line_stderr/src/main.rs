use std::env;
use std::fs;

struct File_line<'a> {
    line: &'a str,
    line_num: u32,
}

fn search<'a>(query: &str, content: &'a str) -> Vec<File_line<'a>> {
    let mut res = Vec::new();
    let mut i = 0u32;
    for l in content.lines() {
        i += 1;
        if l.contains(query) {
            let line = File_line {
                line: l,
                line_num: i,
            };
            res.push(line);
        }
    }
    res
}

fn main() {
    // Arguments that has been passed with execution can get from std::env::args()
    // Example input would be as below.
    // cargo run -p command_line_stderr -- frog hello.txt
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        // eprint function prints it on the shell.
        // It will be considered as an error output (stderr).
        eprintln!("Usage : 19_command_line search_string file_name");
        return;
    }

    let search_string = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path).expect("Can not read file.");

    let res = search(search_string, &contents);

    if res.len() == 0 {
        println!("No result found");
    } else {
        println!("Search result is as follows.");
        for l in res {
            println!("line {0} : {1}", l.line_num, l.line);
        }
    }

    println!("Searching for path variable.");

    //Environment variable also can be optained by env.
    let path = env::var("PATH");
    if path.is_err() {
        eprintln!("Path variable not found");
        return;
    }
    let mut path = path.unwrap();
    let mut path = path.as_str();
    if path.len() > 20 {
        path = &path[..20]
    }
    println!("Path contains {path}.. (so on)");

    //If it doesn't contain any thing, it will gives
    let path = env::var("Unexpected environment");
    if path.is_err() {
        eprintln!("Unexpected environment variable not found");
        return;
    }
}
