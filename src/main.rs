use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io;

// The rust book
fn main() {
    // to remove warning for unused function, put in false if statement
    chp91();
    chp84();
    if false {
        chp83();
        chp82();
        chp81();
        enum1();
        enum2();
        struct1();
        struct_test_1();
        ownership_borrow();
        str_literal();
        fib_recc(1);
        fib_while(1);
        guess_the_number();

        const COUNT: u32 = 10;
        for num in 0..COUNT {
            println!("{}", fib(num));
        }
    }
}

// Chapter 2: Programming a Guessing Game
fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// Chapter 3: Common Programming Concepts
fn fib_recc(num: u32) -> u32 {
    if num == 1 || num == 0 {
        1
    } else {
        fib_recc(num - 1) + fib_recc(num - 2)
    }
}

fn fib(num: u32) -> u32 {
    if num == 1 || num == 0 {
        return 1;
    }
    let mut a1: u32 = 1;
    let mut a2: u32 = 1;
    for _ in 1..num {
        let a3 = a2;
        a2 = a2 + a1;
        a1 = a3;
    }
    a2
}

// TODO
fn fib_while(num: u32) -> u32 {
    if num == 1 || num == 0 {
        return 1;
    }
    let mut a1: u32 = 1;
    let mut a2: u32 = 1;
    for _ in 1..num {
        let a3 = a2;
        a2 = a2 + a1;
        a1 = a3;
    }
    a2
}

// Chapter 4: Ownership (manage member)
fn str_literal() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}

fn string() -> String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    s // we return a reference to the String, s
}

fn read_only_ref_str(s: &String) {
    println!("{}", s)
}
fn read_only_str(s: String) {
    println!("{}", s)
}
fn ref_mutate(s: &mut String) {
    s.push_str(" mutated")
}

// chp 4
fn ownership_borrow() {
    let s = string();
    println!("{},{}", string(), s);
    println!("{},{}", string(), s);
    read_only_ref_str(&s); // s is not borrowed

    // s is still available here

    let mut s = string(); // can shadow without issue

    ref_mutate(&mut s); // mutated ref

    // s is still available here

    read_only_str(s); // s is borrowed

    // s is no longer available here
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn struct_test_1() {
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("Create normal user");

    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("Create mutate user and use shadowing to replace variable");

    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("anotheremail@example.com");
    user1.active = true;
    user1.sign_in_count = 0;
    println!("mutate user using dot notation");

    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("shorthand to update");
}

fn struct1() {
    let rect1 = Rectangle {
        width: 1,
        height: 2,
    };

    println!("{}", area(&rect1));
    println!("{}", rect1.area());
    println!("{:?}", rect1);
    // dbg!(&rect1);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let _sq = Rectangle::square(3);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn enum1() {
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
    let _absent_number: Option<i32> = None;
}
// enum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum2() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alaska);
    let quarter2 = Coin::Quarter(UsState::Alabama);
    value_in_cents(penny);
    value_in_cents(nickel);
    value_in_cents(dime);
    value_in_cents(quarter);
    value_in_cents(quarter2);
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        // Nickel | Dime | Quarter => todo!()
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Chapter 8: common collections
fn chp81() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn chp82() {
    let mut s = String::new();
    s.push_str("s");
    println!("{}", s);
    let data = "initial contents";

    let s = data.to_string();
    s.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    let mut s1 = String::from("foo");
    let s2 = "bar"; //&str so it is already borrowed
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // invalid
    // let hello = "Здравствуйте";
    // let answer = &hello[0];
}

// fn fn1() {}
fn chp83() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // get value from key in hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // insert on value with null
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // change value base on existing
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn chp84() {
    // let a = vec![1, 2, 3, 4, 5];
    // let a = vec![1, 2, 3, 4, 5, 6];
    let a = vec![1, 2, 3, 4, 5, 6, 6, 7, 7];
    let a1 = a.len();
    let a2 = a.len() / 2;
    let median = if a1 % 2 == 1 {
        a[a2] as f64
    } else {
        (a[a2] + a[a2 - 1]) as f64 / 2.0
    };
    println!("Median: {}", median);

    let mut map = HashMap::new();

    let mut max_count = 1;
    let mut max = vec![a[0]];
    let first_index = a[0];

    for int in a {
        let count = map.entry(int).or_insert(0);
        *count += 1;
        if int == first_index {
            continue;
        }
        if count > &mut max_count {
            // impossible condition to remove all elements
            max.retain(|&x| x * 0 == 1);
            max.push(int);
            max_count = *count;
        } else if count == &mut max_count {
            max.push(int);
            max_count = *count;
        }
        // get mode here
    }
    // println!("{}", map);
    println!("Map count: {:?}", map);
    println!("Mode(s): {:?}", max);
    println!("Max count: {:?}", max_count);
}

fn chp91() {}
