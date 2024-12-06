fn main() {
  let array = [1, 2, 3, 4, 5];

  // println!("Array {{}}: {}", array); // cannot be formatted with the default formatter
  println!("Array {{:?}}: {:?}", array);
  println!("Array {{:#?}}: {:#?}", array);

  let tuple = (1, 2, 3, 4, 5);

  // println!("Tuple {{}}: {}", tuple); // cannot be formatted with the default formatter
  println!("Tuple {{:?}}: {:?}", tuple);
  println!("Tuple {{:#?}}: {:#?}", tuple);

  let dict = vec![("key1", "value1"), ("key2", "value2"), ("key3", "value3")];

  // println!("Dict {{}}: {}", dict); // cannot be formatted with the default formatter
  println!("Dict {{:?}}: {:?}", dict);
  println!("Dict {{:#?}}: {:#?}", dict);
}
