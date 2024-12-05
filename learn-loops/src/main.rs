fn main() {
  let n = 10;
  
  for i in 0..n {
    println!("for i in 0..{}: {}", n, i);
  }

  for i in 0..=n {
    println!("for i in 0..={}: {}", n, i);
  }

  let mut while_i = 0;
  
  while while_i < n {
    println!("while i < {}: {}", n, while_i);
    while_i += 1;
  }

  let mut loop_i = 0;

  loop {
    println!("loop: {}", loop_i);
    loop_i += 1;

    if loop_i >= n { break; }
  }

  let mut array = [2, 1, 3, 4, 5];

  for item in &array {
    println!("for item in &array: - {}?", item);
  }

  array.iter().for_each(|x| println!("foreach: - {}?", x));

  for (index, value) in array.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
  }

  array.iter_mut().for_each(|x| *x += 1);
  println!("Updated array: {:?}", array);

  array.iter().enumerate().for_each(
    |(index, value)| println!("Index: {}, Value: {}", index, value)
  );

  array.iter_mut().enumerate().for_each(
    |(index, value)| *value += index
  );

  println!("Updated array 2: {:?}", array);

  for item in &mut array {
    *item += 1;
  }

  println!("Updated array 3: {:?}", array);

  let sum = (0..=n).sum::<i32>();
  println!("Sum of (0..={}): {}", n, sum);

  // advanced
  let fold = (0..=n).fold(0, |result, x| result + x);
  println!("Fold of (0..={}): {}", n, fold);

  // fold takes an initial value and a closure
  // 0 - initial value (result at the first iteration)
  // |result, x| - closure
  // result - accumulator (short: acc)
}