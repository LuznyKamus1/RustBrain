use std::{env, fs};

fn main() {
    let table_size = 255;

    let args: Vec<String> = env::args().collect();
    println!("reading: {}", args[1]);

    let file: Vec<char> = fs::read_to_string(&args[1])
        .expect("cannot read file!")
        .chars()
        .collect();

    let mut table: Vec<usize> = vec![0; table_size];
    let mut pointer: usize = 0;

    for x in 0..file.len() {
        match file[x] {
            '<' if !(pointer<=0) => pointer-=1,
            '>' if !(pointer>=table_size) => pointer+=1,
            '+' => table[pointer]+=1,
            '-' => table[pointer]-=1,
            '!' => println!("{}", table[pointer].to_string()),
            //'[' if table[pointer]==0 => x= jump_to_whilend(&file, x),
            //']' => x=jump_towhilestart(&file, x),
            _ => ()
        }
    }

}
//fn jump_to_whilend(f: &Vec<char>, mut p: usize) -> usize { while f[p] != ']' {p+=1}return p+1;}
//fn jump_towhilestart(f: &Vec<char>, mut p: usize) -> usize { if p!=0 {while f[p] != '[' {p-=1}}return p;}
//
//TODO: LOOP, PRINT CHARS
