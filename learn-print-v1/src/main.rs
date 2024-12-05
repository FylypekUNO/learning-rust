fn main() {
  println!("Hello, world!");

  let string = "Hello, world!";
  println!("{} {:?}", string, string);

  let number = 10;
  println!("num: {}", number);

  let boolean = true;
  println!("bool: {} - {}", boolean, boolean as u8);

  let float = 3.141592653589793;
  println!("float: {} {:?} {:.4} {:e} {:.2e}", float, float, float, float, float);

  let precision = 3;
  println!("float with precision: {:.prec$}", float, prec = precision);
}