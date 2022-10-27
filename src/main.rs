use std::{time::{Instant}, cmp::Ordering};
use rand::Rng;


fn main() {

    let msg = String::from("Hello fellow Rustaceans!");
    let start = Instant::now();
    let count = msg.chars().count();
    let mut count2 = 0;
    for c in 1..10_000  {
        count2 += c -c + msg.chars().count() / 100
    }
    let duration = start.elapsed();

    println!("Time elapsed counting {:?} chars {} times, of {} was: {:?}", count, count2, msg, duration);

    #[allow(dead_code)]
    #[derive(Debug)]
    struct SimpleStructure(i32);
    struct Structure{attribute1: i32, attribute2: String}

    println!("This struct `{:?}` print using the Debug trait and Elvis...", SimpleStructure(3));
    
    impl std::fmt::Display for Structure {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "(attr1: {}, attr2: {})", self.attribute1, &self.attribute2)
        }
    }
    let s = Structure {attribute1: 4, attribute2: String::from("sadf")};
    println!("This struct `{}` print after manually implementing the display trait, then Elvis aint necessary...", s);

    let attribute1 = 4;
    let attribute2 = String::from("sadf");
    let _s2 = Structure {attribute1, attribute2}; // only works, if var names match field names :-(  ; note the underscore at the start to ignore the error that the var aint used

    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("random number {}", random_number);

    let middle: i32 = "50 ".trim().parse().expect("not an int32");

    // alternatively, rather than expect, use this:
    let middle2: i32 = match "middle2".trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("oops");
            50 // default value to use. last line with no semi colon is the return value
        }, 
    };
    println!("midle2: {}", middle2);

    match middle.cmp(&random_number).reverse() {
        Ordering::Less => println!("less!"),
        Ordering::Greater => println!("more!"),
        Ordering::Equal => println!("equal!"),
    }




}
