use mars_rover::Rover;

#[test]
fn initial_position_at_origin_facing_north() {
    let rover = Rover::new();
    let position = rover.execute("");
    assert_eq!(position, "0:0:N")
}