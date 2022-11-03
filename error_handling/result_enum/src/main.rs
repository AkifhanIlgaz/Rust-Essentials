use std::fs;
use std::io;
fn main() {
    let result = fs::read_to_string(
        "c:\\Users\\AKIF\\Desktop\\Let's Get Rusty\\exercise\\the_ultimate_question.txt",
    );

    let contents = match result {
        Ok(content) => content,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("file not found"),
            io::ErrorKind::PermissionDenied => String::from("permission denied"),
            _ => panic!("Another type of error: {:?}", error),
        },
    };

    println!("Contents are: {contents}");
}
