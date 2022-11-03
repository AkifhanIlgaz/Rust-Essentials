use std::{fs, io};

fn read_and_combine(file1: &str, file2: &str) -> Result<String, io::Error> {
    let mut s1 = fs::read_to_string(file1)?;

    let s2 = match fs::read_to_string(file2) {
        Ok(text) => text,
        Err(error) => return Err(error),
    };

    s1.push('\n');
    s1.push_str(&s2);

    Ok(s1)
}

fn main() {
    let result = read_and_combine(
        "c:\\Users\\AKIF\\Desktop\\Let's Get Rusty\\exercise\\the_ultimate_question.txt",
        "c:\\Users\\AKIF\\Desktop\\Let's Get Rusty\\exercise\\the_ultimate_question.txt",
    );

    match result {
        Ok(s) => println!("result is... \n{}", s),
        Err(e) => println!("there was an error: {}", e),
    }
}
