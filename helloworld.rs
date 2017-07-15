fn main() {
  /// I'm part of documentation but will be ignored by the compiler
  ////! ```
  ////!  me too
  ////!  mee too
  ////!  meee too
  ////! ```

  println!("{0} {1}!", "Hello", "World");

  // As can named arguments.
  println!("{subject} {verb} {object}",
           object="the lazy dog",
           subject="the quick brown fox",
           verb="jumps over");
           // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // It will even check to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure which contains an `i32`. Name it `Structure`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    // print "Pi is roughly 2.142" by controlling the number of decimal places shown
    let pi = 3.141586254;
    println!("{:.*}", 2, pi);
}
