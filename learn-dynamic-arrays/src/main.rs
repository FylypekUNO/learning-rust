fn accumulate_string(mut acc: String, s: &&str) -> String {
  acc.push_str(s);
  acc
}

fn sum_string(strings: &[&str]) -> String {
  let capacity = strings.iter().fold(0, |acc, s| acc + s.len());

  strings
    .iter()
    .fold(String::with_capacity(capacity), accumulate_string)
}

fn main() {
  let mut strings: Vec<&str> = vec!["one", "two", "three"];

  let sum = sum_string(&strings);

  let sum2 = strings.join(", ");

  println!("{}", sum);
  println!("{}", sum2);

  strings.remove(1);

  let new_sum = sum_string(&strings);
  println!("{}", new_sum);

  let strings = vec!["one", "two", "three"];

  let filtered_strings = strings
    .iter()
    .filter(|s| s.len() > 4)
    .copied()
    .collect::<Vec<&str>>();

  let filtered_sum = sum_string(&filtered_strings);
  println!("{}", filtered_sum);
}
