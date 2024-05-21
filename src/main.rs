use std::fs;
use std::env;
use std::vec;

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    Ok(data)
}

fn validate_args(args: &Vec<String>) {
    if args.len() != 2 {
        panic!("Usage: cargo run -- <file_path>");
    }
    if !args[1].ends_with(".meow") {
        panic!("The input file must end with the extension '.meow'");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    validate_args(&args);

    let file: Vec<u8> = match read_file_vec(&args[1]) {
        Ok(vec) => vec,
        Err(_) => {
            panic!("Error: validate the path of your file");
        }
    };

    for &u_char in &file {
        println!("{}", u_char as char);
    }
    
    // let mut ast: Vec<Vec<Vec<String>>> = Vec::new();
    // let mut curr_branch: Vec<Vec<String>> = vec![Vec::new()];
    // let mut curr_word: Vec<char> = Vec::new();

    // for &u_char in &file {
    //     match u_char {
    //         b';' => {
    //             if !curr_word.is_empty() {
    //                 curr_branch.last_mut().unwrap().push(curr_word.iter().collect());
    //                 curr_word.clear();
    //             }
    //         },
    //         b'{' => {
    //             curr_branch.push(Vec::new());
    //         },
    //         b'}' => {
    //             let sub_branch = curr_branch.pop().unwrap();
    //             curr_branch.last_mut().unwrap().push(sub_branch);
    //         },
    //         b'\n' => continue,
    //         _ => {
    //             if u_char == b' ' || u_char == b'=' {
    //                 if !curr_word.is_empty() {
    //                     curr_branch.last_mut().unwrap().push(curr_word.iter().collect());
    //                     curr_word.clear();
    //                 }
    //             } else {
    //                 curr_word.push(u_char as char);
    //             }
    //         },
    //     }
    // }

    // if !curr_word.is_empty() {
    //     curr_branch.last_mut().unwrap().push(curr_word.iter().collect());
    // }

    // println!("{:?}", ast);
}
