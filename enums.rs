enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // player_direction is of type Direction and the value is the value Up
    let player_direction:Direction = Direction::Up;

    // works like switch
    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are heading Down!"),
        Direction::Left => println!("We are heading Left!"),
        Direction::Right => println!("We are heading Right!"),
    }
}