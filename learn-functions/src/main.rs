fn do_something(x: i32) {
  println!("do_something: {}", x);
}

fn multiply(x: i32, y: i32) -> i32 {
  return x * y;
}

// take array of ints, return min
fn min(array: &[i32]) -> i32 {
  return array
    .iter()
    .skip(1)
    .fold(array[0], |min, &x| if x < min { x } else { min });
}

fn max(array: &[i32]) -> i32 {
  return array
    .iter()
    .skip(1)
    .fold(array[0], |max, &x| if x > max { x } else { max });
}

fn min_max(array: &[i32]) -> (i32, i32) {
  return (min(array), max(array));
}

fn plus_one(array: &[i32]) -> Vec<i32> {
  let mut result = Vec::with_capacity(array.len());

  result.extend(array.iter().map(|x| x + 1));

  return result;
}

fn main() {
  let x = 10;
  do_something(x);

  let y = 20;

  let z = multiply(x, y);
  do_something(z);

  let array = [1, 2, 3, 4, 5];
  let (min, max) = min_max(&array);

  let added_array = plus_one(&array);
  let (added_min, added_max) = min_max(&added_array);

  println!("min: {}, max: {}", min, max);
  println!("addedMin: {}, addedMax: {}", added_min, added_max);
}
