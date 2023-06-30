use std::{env, fs};

fn main() {
    let table_size: i32 = 256;

    let args: Vec<String> = env::args().collect();
    println!("reading: {}", args[1]);

    let file: Vec<char> = fs::read_to_string(&args[1])
        .expect("cannot read file!")
        .chars()
        .collect();

    let mut table: Vec<i32> = vec![0; table_size as usize];
    let mut pointer: i32 = 0;

    let mut current_char: usize = 0;

    while current_char < file.len() {
        match file[current_char] {
            '<' if !(pointer<=0) => pointer-=1,
            '>' if !(pointer>=table_size) => pointer+=1,
            '+' => table[pointer as usize]+=1,
            '-' => table[pointer as usize]-=1,
            '[' => current_char = start_loop(&file, current_char, &table, pointer),
            ']' => current_char = end_loop(&file, current_char, &table, pointer),
            '!' => println!("{}", table[pointer as usize].to_string()),
            _ => ()
        }
        current_char+=1;
    }
}

fn start_loop(file: &Vec<char>, mut current_char: usize, table: &Vec<i32>, pointer: i32) -> usize {
    if table[pointer as usize]==0 {
        while file[current_char]!=']' {current_char+=1;}
        return current_char;
    } else {return current_char;}
}
fn end_loop(file: &Vec<char>, mut current_char: usize, table: &Vec<i32>, pointer: i32) -> usize {
    if table[pointer as usize]!=0 {
        while file[current_char]!='[' {current_char-=1;}
        return current_char;
    } else {return current_char;}
}
