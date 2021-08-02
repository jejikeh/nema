use std::io;

fn main() {


    let mut s1 = String::new();

    io::stdin()
        .read_line(&mut s1)
        .expect("Fuck py");

    let len = calculate_length(&s1);

    println!("The fuck you is {}",len-2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
