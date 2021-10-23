use std::io;

fn main() {
    println!("Starting the basic ECS implementation");

    io::Write::flush(&mut io::stdout());
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);
    print!("{}", buffer);
    buffer.clear();

}
