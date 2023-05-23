pub fn fizzbuzz (input: u32) -> String {
  let is_divisible_by_three: bool = input % 3 == 0;
  let is_divisible_by_five: bool = input % 5 == 0;
  match (is_divisible_by_three, is_divisible_by_five) {
    (true, false) => String::from("Fizz"),
    (false, true) => String::from("Buzz"),
    (false, false)=> { input.to_string() }
    (true, true) => String::from("FizzBuzz")
  }
}
