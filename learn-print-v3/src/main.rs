fn main() {
  let string = "Hello";

  println!("{{}}:        {}", string);
  println!("{{:?}}:      {:?}", string);
  println!("{{:#?}}:     {:#?}", string);

  println!("{{:*<10}}:   {:*<10}", string);
  println!("{{:*^10}}:   {:*^10}", string);
  println!("{{:*>10}}:   {:*>10}", string);

  println!("{{:>10.2}}:  {:>10.2}", string);
  println!("{{:>10.4}}:  {:>10.4}", string);
  println!("{{:>10.10}}: {:>10.10}", string);

  println!();

  let fixed_size = 10;
  println!("{{:>fixed_size$}}: {:>fixed_size$}", string, fixed_size = fixed_size);

  let array = [
    "Hello",
    "World",
    "Rust",
  ];

  let formatted = array
    .iter() // switch from array to iterator
    .map(|x| format!("- {}", x)) // JavaScript: (x) => `- ${x}`
    .collect::<Vec<_>>() // switch from iterator back to array
    .join("\n");

  println!("\nFormatted array:\n{}", formatted);
}