use mars_rover::Rover;
use rstest::rstest;

#[test]
fn initial_position_at_origin_facing_north() {
    let mut rover = Rover::new();
    let position = rover.execute("");
    assert_eq!(position, "0:0:N")
}
#[rstest]
#[case::rotate_to_east("R", 'E')]
#[case::rotate_to_south("RR", 'S')]
#[case::rotate_to_west("RRR", 'W')]
#[case::rotate_to_north("RRRR", 'N')]
#[case::rotate_back_to_east("RRRRR", 'E')]
fn it_rotates_to_the_right(#[case] commands: &str, #[case] cardinal:char ) {
    let mut rover = Rover::new();
    let position = rover.execute(commands);
    assert_eq!(position, format!("0:0:{}", cardinal))
}

#[rstest]
#[case::rotate_to_west("L", 'W')]
#[case::rotate_to_south("LL", 'S')]
#[case::rotate_to_east("LLL", 'E')]
#[case::rotate_to_north("LLLL", 'N')]
#[case::rotate_back_to_west("LLLLL", 'W')]
fn it_rotates_to_the_left(#[case] commands: &str, #[case] cardinal:char ) {
    let mut rover = Rover::new();
    let position = rover.execute(commands);
    assert_eq!(position, format!("0:0:{}", cardinal))
}

#[rstest]
#[case::moves_forward("M", "0:1:N")]
#[case::moves_forward("MM", "0:2:N")]
#[case::moves_forward("MMMMMMMMMMM", "0:0:N")]
fn it_moves_forward(#[case] commands: &str, #[case] expected_position: &str) {
    let mut rover = Rover::new();
    let position = rover.execute(commands);
    assert_eq!(position, format!("{}", expected_position))
}

#[rstest]
#[case::turns_east_and_moves_forward("RM", "1:0:E")]
#[case::turns_east_and_moves_forward_twice("RMM", "2:0:E")]
#[case::turns_east_and_moves_forward_eleven("RMMMMMMMMMMM", "0:0:E")]
fn it_turns_east_and_moves_forward(#[case] commands: &str, #[case] expected_position: &str) {
    let mut rover = Rover::new();
    let position = rover.execute(commands);
    assert_eq!(position, format!("{}", expected_position))
}

#[rstest]
#[case::turns_west_and_moves_forward("LM", "10:0:W")]
#[case::turns_west_and_moves_forward_twice("LMM", "9:0:W")]
#[case::turns_west_and_moves_forward_eleven("LMMMMMMMMMMM", "0:0:W")]
fn it_turns_west_and_moves_forward(#[case] commands: &str, #[case] expected_position: &str) {
    let mut rover = Rover::new();
    let position = rover.execute(commands);
    assert_eq!(position, format!("{}", expected_position))
}

#[rstest]
#[case::turns_south_and_moves_forward("LLM", "0:10:S")]
#[case::turns_south_and_moves_forward_twice("LLMM", "0:9:S")]
#[case::turns_south_and_moves_forward_eleven("LLMMMMMMMMMMM", "0:0:S")]
fn it_turns_south_and_moves_forward(#[case] commands: &str, #[case] expected_position: &str) {
    let mut rover = Rover::new();
    let position = rover.execute(commands);
    assert_eq!(position, format!("{}", expected_position))
}