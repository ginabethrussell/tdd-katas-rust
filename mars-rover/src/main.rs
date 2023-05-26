use mars_rover::Rover;

fn main() {
    println!("Hello, robot driver!");
    let mut rover = Rover::new();
    let position = rover.execute("MMRMMLM");
    println!("End position is: {}", position)
}
