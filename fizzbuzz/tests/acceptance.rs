use fizzbuzz::fizzbuzz;

#[test]
fn one_returns_one() {
    let result: String = fizzbuzz(1);
    assert_eq!(result, String::from("1"))
}
#[test]
fn three_returns_fizz() {
  let result: String = fizzbuzz(3);
  assert_eq!(result, String::from("Fizz"))
}

#[test]
fn six_returns_fizz() {
  let result: String = fizzbuzz(6);
  assert_eq!(result, String::from("Fizz"))
}

#[test]
fn five_returns_buzz() {
  let result: String = fizzbuzz(5);
  assert_eq!(result, String::from("Buzz"))
}

#[test]
fn fifteen_returns_fizzbuzz() {
  let result: String = fizzbuzz(15);
  assert_eq!(result, String::from("FizzBuzz"))
}