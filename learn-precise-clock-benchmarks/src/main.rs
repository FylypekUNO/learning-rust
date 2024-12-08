use std::time::{Duration, Instant};

fn benchmark<F>(mut func: F, iterations: u32) -> Duration
where
  F: FnMut(),
{
  let start = Instant::now();
  for _ in 0..iterations {
    func();
  }
  start.elapsed() / iterations
}

fn benchmark_and_print<F>(name: &str, func: F, iterations: u32)
where
  F: FnMut(),
{
  let duration = benchmark(func, iterations);
  println!("{:<20}: {:.9}s", name, duration.as_secs_f64());
}

// Benchmarks

fn benchmark_1(iterations: u32) {
  let mut numbers_vec = Vec::new();
  for _ in 0..100 {
    numbers_vec.push(0);
  }
  let numbers_array = [0; 100];

  benchmark_and_print(
    "vec[]",
    || {
      for i in 0..numbers_vec.len() {
        let _number = numbers_vec[i];
      }
    },
    iterations,
  );

  benchmark_and_print(
    "array[]",
    || {
      for i in 0..numbers_array.len() {
        let _number = numbers_array[i];
      }
    },
    iterations,
  );
}

fn benchmark_2(iterations: u32, test_capacity: usize) {
  benchmark_and_print(
    "fill vec (no cap)",
    || {
      let mut new_numbers_vec = Vec::new();

      for _ in 0..test_capacity {
        new_numbers_vec.push(0);
      }
    },
    iterations,
  );

  benchmark_and_print(
    "fill vec (with cap)",
    || {
      let mut new_numbers_vec = Vec::with_capacity(test_capacity);

      for _ in 0..test_capacity {
        new_numbers_vec.push(0);
      }
    },
    iterations,
  );
}

fn benchmark_large_arrays() {
  let size = 10_000_000;

  // Method 1: vec! macro
  let start = Instant::now();
  let _v1 = vec![0; size];
  println!("vec! macro: {:.9}s", start.elapsed().as_secs_f64());

  // Method 2: Vec::with_capacity + extend
  let start = Instant::now();
  let mut v2 = Vec::with_capacity(size);
  v2.extend(std::iter::repeat(0).take(size));
  println!("extend method: {:.9}s", start.elapsed().as_secs_f64());

  // Method 3: Vec::from_iter
  let start = Instant::now();
  let _v3: Vec<i32> = std::iter::repeat(0).take(size).collect();
  println!("collect method: {:.9}s", start.elapsed().as_secs_f64());
}

fn main() {
  benchmark_1(1_000_000);

  println!();

  benchmark_2(1_000, 10_000);

  println!();

  let start_1 = Instant::now();
  let mut numbers_vec: Vec<i32> = Vec::new();
  println!("Vec::new() took {:.9}s", start_1.elapsed().as_secs_f64());

  let test_capacity = 10_000_000;

  let start_2 = Instant::now();
  for _ in 0..test_capacity {
    numbers_vec.push(0);
  }
  println!(
    "fill non-cap Vec took {:.9}s",
    start_2.elapsed().as_secs_f64()
  );

  println!("actual capacity: {}", numbers_vec.capacity());
  let start_99 = Instant::now();
  numbers_vec.shrink_to_fit();
  println!(
    "shrink_to_fit() took {:.9}s",
    start_99.elapsed().as_secs_f64()
  );
  println!("new capacity: {}", numbers_vec.capacity());

  let start_3 = Instant::now();
  let mut numbers_vec_2: Vec<i32> = Vec::with_capacity(test_capacity);
  println!(
    "Vec::with_capacity() took {:.9}s",
    start_3.elapsed().as_secs_f64()
  );

  let start_4 = Instant::now();
  for _ in 0..test_capacity {
    numbers_vec_2.push(0);
  }
  println!("fill cap Vec took {:.9}s", start_4.elapsed().as_secs_f64());

  println!();

  benchmark_large_arrays();
}
