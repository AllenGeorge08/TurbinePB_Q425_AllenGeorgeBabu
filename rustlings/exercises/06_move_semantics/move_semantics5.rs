#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    //e Added the & borrow here
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    //e Took the ownership here... Took the ownership here by removing the borrow reference...
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
