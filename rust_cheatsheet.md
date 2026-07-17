# Rust Cheat Sheet

A systems programming language focused on safety, speed, and concurrency without garbage collection, using ownership and borrowing to guarantee memory safety at compile time.

---

## Core Concepts

Rust achieves memory safety without garbage collection through its unique ownership system. The compiler enforces strict rules at compile time to prevent common bugs.

- **Ownership**: Each value has a single owner, and values are dropped when owner goes out of scope
- **Borrowing**: References allow temporary access without transferring ownership
- **Lifetimes**: Annotations that ensure references are always valid
- **Move Semantics**: Values are moved by default, preventing double-free errors
- **Zero-Cost Abstractions**: High-level features with no runtime overhead
- **Immutability by Default**: Variables are immutable unless explicitly marked as mutable
- **Pattern Matching**: Exhaustive matching that the compiler verifies
- **Traits**: Interface-like definitions for shared behavior
- **Cargo**: Built-in package manager and build system
- **Memory Safety**: No null pointers, no data races, no buffer overflows at compile time

---

## Key Terms / Definitions

- **Ownership**: System where each value has one owner that controls its lifetime
- **Borrow**: Temporary reference to a value without taking ownership
- **Lifetime**: Scope for which a reference is valid
- **Move**: Transfer of ownership from one variable to another
- **Copy**: Types that can be duplicated by copying bits (integers, bools, etc.)
- **Clone**: Explicit deep copy of data
- **Trait**: Collection of methods defining behavior, similar to interfaces
- **Struct**: Custom data type that groups related values
- **Enum**: Type that can be one of several variants
- **Option**: Enum representing optional values (`Some(T)` or `None`)
- **Result**: Enum representing success (`Ok(T)`) or failure (`Err(E)`)
- **Closure**: Anonymous function that can capture its environment
- **Iterator**: Trait for types that can produce sequence of values
- **Macro**: Code that writes code at compile time (ends with `!`)
- **Crate**: Package of Rust code (library or binary)
- **Module**: Namespace for organizing code within a crate

---

## Variables & Mutability

Variables in Rust are immutable by default, which prevents accidental modifications and enables compiler optimizations. Mutability must be explicitly declared.
```rust
// Immutable variable (default)
let x = 5;
// x = 6; // Error: cannot assign twice to immutable variable

// Mutable variable
let mut y = 5;
y = 6; // OK

// Constants (always immutable, type must be annotated)
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

// Shadowing (redeclare variable with same name)
let x = 5;
let x = x + 1; // x is now 6
let x = "hello"; // Can change type with shadowing

// Type annotations
let x: i32 = 5;
let y: f64 = 3.14;
let name: &str = "Alice";

// Multiple variable declaration
let (a, b, c) = (1, 2, 3);
```

---

## Data Types

Rust is statically typed, meaning all types must be known at compile time. The compiler can often infer types, but annotations are required when multiple types are possible.

### Scalar Types
```rust
// Integers (signed: i8, i16, i32, i64, i128, isize)
let x: i32 = -42;
let y: u32 = 42; // unsigned: u8, u16, u32, u64, u128, usize

// Integer literals
let decimal = 98_222;
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A'; // u8 only

// Floating point
let x: f32 = 3.14;
let y: f64 = 2.71828; // default

// Boolean
let is_true: bool = true;
let is_false = false;

// Character (4 bytes, Unicode)
let c: char = 'z';
let emoji = '😊';
let chinese = '中';
```

### Compound Types
```rust
// Tuple (fixed size, mixed types)
let tup: (i32, f64, char) = (500, 6.4, 'x');
let (x, y, z) = tup; // destructuring
let first = tup.0; // access by index

// Array (fixed size, same type)
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];
let repeated = [3; 5]; // [3, 3, 3, 3, 3]

// Slice (reference to contiguous sequence)
let slice: &[i32] = &arr[1..3]; // [2, 3]
let slice = &arr[..]; // entire array

// String slice
let s = "hello";
let slice = &s[0..2]; // "he"

// String (heap-allocated, growable)
let mut s = String::from("hello");
s.push_str(", world!");
let len = s.len();

// Vector (growable array)
let mut v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3]; // macro for initialization
v.push(4);
let third = v[2];
let third = v.get(2); // returns Option<&i32>
```

---

## Ownership Rules

Ownership is Rust's most unique feature, enabling memory safety without garbage collection. These rules are enforced at compile time.

### The Three Rules

1. Each value in Rust has a variable that's called its owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value is dropped
```rust
// Ownership example
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2, s1 is no longer valid
// println!("{}", s1); // Error: value borrowed after move
println!("{}", s2); // OK

// Copy types (stack-only data)
let x = 5;
let y = x; // x is copied, both are valid
println!("{} {}", x, y); // OK

// Clone (explicit deep copy)
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2); // Both valid

// Functions and ownership
fn takes_ownership(s: String) {
    println!("{}", s);
} // s is dropped here

fn makes_copy(x: i32) {
    println!("{}", x);
} // x goes out of scope, nothing special happens

let s = String::from("hello");
takes_ownership(s); // s is moved
// println!("{}", s); // Error

let x = 5;
makes_copy(x); // x is copied
println!("{}", x); // OK

// Return values transfer ownership
fn gives_ownership() -> String {
    String::from("hello")
}

fn takes_and_gives_back(s: String) -> String {
    s // returned and ownership moved
}

let s1 = gives_ownership();
let s2 = String::from("world");
let s3 = takes_and_gives_back(s2);
// s2 is no longer valid
```

---

## References & Borrowing

References allow you to refer to a value without taking ownership. Borrowing rules prevent data races at compile time.

### Borrowing Rules

1. At any time, you can have either one mutable reference OR any number of immutable references
2. References must always be valid (no dangling references)
```rust
// Immutable reference
let s1 = String::from("hello");
let len = calculate_length(&s1); // borrow s1
println!("{} has length {}", s1, len); // s1 still valid

fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but doesn't own the data

// Mutable reference
let mut s = String::from("hello");
change(&mut s);

fn change(s: &mut String) {
    s.push_str(", world");
}

// Multiple immutable references OK
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2); // OK

// Cannot have mutable and immutable references simultaneously
let mut s = String::from("hello");
let r1 = &s; // OK
let r2 = &s; // OK
// let r3 = &mut s; // Error: cannot borrow as mutable
println!("{} {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // OK now
println!("{}", r3);

// Only one mutable reference at a time
let mut s = String::from("hello");
let r1 = &mut s;
// let r2 = &mut s; // Error: cannot borrow as mutable more than once
println!("{}", r1);

// Dangling references prevented
// fn dangle() -> &String { // Error: missing lifetime specifier
//     let s = String::from("hello");
//     &s // s will be dropped, reference would be invalid
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership is moved out
}
```

---

## Structs

Structs are custom data types that group related values. They're similar to classes in other languages but without inheritance.
```rust
// Basic struct
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// Create instance
let user1 = User {
    email: String::from("user@example.com"),
    username: String::from("user123"),
    age: 25,
    active: true,
};

// Access fields
println!("{}", user1.email);

// Mutable struct
let mut user2 = User {
    email: String::from("another@example.com"),
    username: String::from("user456"),
    age: 30,
    active: true,
};
user2.email = String::from("newemail@example.com");

// Struct update syntax
let user3 = User {
    email: String::from("new@example.com"),
    ..user1 // copy remaining fields from user1
};

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// Unit-like structs (no fields)
struct AlwaysEqual;
let subject = AlwaysEqual;

// Methods
impl User {
    // Associated function (no self)
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            age: 0,
            active: true,
        }
    }
    
    // Method (takes self)
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
    
    // Method with mutable reference
    fn deactivate(&mut self) {
        self.active = false;
    }
    
    // Method that takes ownership
    fn into_string(self) -> String {
        format!("{}: {}", self.username, self.email)
    }
}

// Usage
let user = User::new(
    String::from("user@example.com"),
    String::from("user789")
);
println!("Adult: {}", user.is_adult());

// Derive common traits
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect); // Debug print
println!("{:#?}", rect); // Pretty debug print
```

---

## Enums & Pattern Matching

Enums define types that can be one of several variants. Pattern matching provides exhaustive handling of all cases.

### Enums
```rust
// Basic enum
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

// Enum with data
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// Enum with different types
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },   // anonymous struct
    Write(String),              // single value
    ChangeColor(i32, i32, i32), // tuple
}

// Methods on enums
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(text) => println!("Write: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: {}, {}, {}", r, g, b),
        }
    }
}

let msg = Message::Write(String::from("hello"));
msg.call();
```

### Option Enum
```rust
// Option is defined in standard library
enum Option<T> {
    Some(T),
    None,
}

// Handling optional values
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

// Must handle None case to access value
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Unwrap methods
let x = Some(5);
let y = x.unwrap(); // panics if None
let z = x.unwrap_or(0); // default value
let w = x.expect("should have value"); // custom panic message

// Using if let
if let Some(value) = some_number {
    println!("Value: {}", value);
}

// Map and other combinators
let doubled = some_number.map(|x| x * 2);
let result = some_number.and_then(|x| Some(x + 1));
```

### Result Enum
```rust
// Result for error handling
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Function returning Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

// Handling Result
match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}

// Propagating errors with ?
fn read_file() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("file.txt")?; // returns error if fails
    Ok(contents)
}

// Unwrap variants
let result = divide(10.0, 2.0).unwrap(); // panics on Err
let result = divide(10.0, 2.0).expect("Division failed");
let result = divide(10.0, 0.0).unwrap_or(0.0);
```

### Pattern Matching
```rust
// Match expression (exhaustive)
let number = 7;

match number {
    1 => println!("One"),
    2 | 3 | 5 | 7 => println!("Prime under 10"),
    4..=9 => println!("Four to nine"),
    _ => println!("Other"), // catch-all
}

// Match with guards
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("Less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}

// Destructuring
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 0, y: 7 };

match point {
    Point { x: 0, y } => println!("On y-axis at {}", y),
    Point { x, y: 0 } => println!("On x-axis at {}", x),
    Point { x, y } => println!("At ({}, {})", x, y),
}

// Match with enums
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String), // state quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {}", state);
            25
        }
    }
}

// If let (when only care about one pattern)
let some_value = Some(3);

if let Some(3) = some_value {
    println!("Three");
}

// While let
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## Collections

Rust's standard library provides several collection types. The most common are vectors, strings, and hash maps.

### Vectors
```rust
// Create vector
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3]; // macro

// Adding elements
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// Accessing elements
let v = vec![1, 2, 3, 4, 5];
let third = &v[2]; // panics if out of bounds
let third = v.get(2); // returns Option<&i32>

match v.get(2) {
    Some(third) => println!("Third: {}", third),
    None => println!("No third element"),
}

// Iterating
let v = vec![100, 32, 57];

// Immutable iteration
for i in &v {
    println!("{}", i);
}

// Mutable iteration
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // dereference to modify
}

// Vector of enums for multiple types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text(String::from("blue")),
];

// Common methods
v.len();
v.is_empty();
v.pop(); // removes and returns last element
v.remove(2); // removes at index
v.clear(); // removes all elements
```

### Strings
```rust
// Create string
let mut s = String::new();
let s = "initial contents".to_string();
let s = String::from("initial contents");

// Appending
let mut s = String::from("foo");
s.push_str("bar"); // append string slice
s.push('!'); // append single char

// Concatenation
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 is moved, s2 is borrowed

// Format macro (doesn't take ownership)
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);

// Indexing not allowed (strings are UTF-8)
// let h = s[0]; // Error

// Slicing (be careful with UTF-8 boundaries)
let hello = "Здравствуйте";
let s = &hello[0..4]; // "Зд" (2 chars, 4 bytes)

// Iterating
for c in "नमस्ते".chars() {
    println!("{}", c); // individual characters
}

for b in "नमस्ते".bytes() {
    println!("{}", b); // raw bytes
}

// Common methods
s.len(); // byte length
s.is_empty();
s.contains("foo");
s.replace("foo", "bar");
s.split_whitespace();
s.trim();
s.to_lowercase();
s.to_uppercase();
```

### Hash Maps
```rust
use std::collections::HashMap;

// Create hash map
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// Create from vectors
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// Accessing values
let team_name = String::from("Blue");
let score = scores.get(&team_name); // returns Option<&V>

match score {
    Some(&s) => println!("Score: {}", s),
    None => println!("Team not found"),
}

// Iterating
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Updating
let mut scores = HashMap::new();

// Overwrite
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); // overwrites

// Only insert if key doesn't exist
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50); // doesn't change

// Update based on old value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1; // dereference to modify
}

println!("{:?}", map); // {"hello": 1, "world": 2, "wonderful": 1}

// Ownership
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");
let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are no longer valid (moved)
```

---

## Error Handling

Rust groups errors into two categories: recoverable (Result) and unrecoverable (panic!). The type system forces you to handle errors.

### Panic (Unrecoverable Errors)
```rust
// Explicit panic
panic!("crash and burn");

// Panic from out of bounds access
let v = vec![1, 2, 3];
v[99]; // panics

// Set panic behavior in Cargo.toml
// [profile.release]
// panic = 'abort' // don't unwind stack

// Backtrace
// RUST_BACKTRACE=1 cargo run
```

### Result (Recoverable Errors)
```rust
use std::fs::File;
use std::io::ErrorKind;

// Handling Result with match
let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating file: {:?}", e),
        },
        other_error => panic!("Problem opening file: {:?}", other_error),
    },
};

// Shortcuts: unwrap and expect
let f = File::open("hello.txt").unwrap(); // panics on error
let f = File::open("hello.txt").expect("Failed to open file"); // custom message

// Propagating errors with ?
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chain with ?
fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Even shorter with fs
use std::fs;

fn read_username_short() -> Result<String, io::Error> {
    fs::read_to_string("username.txt")
}

// ? can also work with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Custom error types
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for MyError {}

fn do_something() -> Result<(), MyError> {
    Err(MyError { details: String::from("Something went wrong") })
}
```

---

## Traits

Traits define shared behavior across types, similar to interfaces in other languages. They enable polymorphism and generic programming.
```rust
// Define a trait
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

// Implement trait for a type
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    // uses default implementation for summarize_default
}

// Using traits
let article = NewsArticle {
    headline: String::from("Breaking News"),
    location: String::from("New York"),
    author: String::from("John Doe"),
    content: String::from("..."),
};

println!("{}", article.summarize());

// Traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax (equivalent to above)
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
pub fn notify(item: &(impl Summary + Display)) {
    // ...
}

// Or with generics
pub fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// Where clause (cleaner for multiple bounds)
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    // ...
}

// Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("user"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    }
}

// Conditional implementations
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest: {}", self.x);
        } else {
            println!("Largest: {}", self.y);
        }
    }
}
```

### Common Standard Library Traits
```rust
// Debug - for {:?} formatting
#[derive(Debug)]
struct Point { x: i32, y: i32 }

// Clone - explicit copy
#[derive(Clone)]
struct MyStruct { data: String }

// Copy - implicit copy (stack-only data)
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

// PartialEq and Eq - equality comparison
#[derive(PartialEq, Eq)]
struct User { id: u32, name: String }

// PartialOrd and Ord - ordering comparison
#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Priority(u8);

// Display - for {} formatting
use std::fmt;

struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// From and Into - type conversion
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

let num = Number::from(30);
let num: Number = 30.into(); // Into is automatic

// Iterator
struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

---

## Generics

Generics enable writing code that works with multiple types without duplication. The compiler generates specific implementations for each type used.
```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

let numbers = vec![34, 50, 25, 100, 65];
let result = largest(&numbers);

let chars = vec!['y', 'm', 'a', 'q'];
let result = largest(&chars);

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };

// Multiple generic types
struct Point<T, U> {
    x: T,
    y: U,
}

let both = Point { x: 5, y: 4.0 };

// Generic enums
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generic methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Method for specific type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Different generics in method
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Zero-cost abstraction - no runtime overhead
// Compiler generates specific versions for each type used (monomorphization)
```
---

## Lifetimes

Lifetimes ensure references are valid for as long as they're used. Most of the time, lifetimes are inferred, but sometimes explicit annotations are needed.
```rust
// Lifetime annotations in function signatures
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The returned reference will be valid for the shorter of x or y
let string1 = String::from("long string");
let result;
{
    let string2 = String::from("short");
    result = longest(string1.as_str(), string2.as_str());
    println!("{}", result); // OK here
}
// println!("{}", result); // Error: string2 doesn't live long enough

// Lifetime in structs
struct ImportantExcerpt<'a> {
    part: &'a str, // reference must live as long as struct
}

let novel = String::from("Call me Ishmael. Some years ago...");
let first_sentence = novel.split('.').next().expect("Could not find '.'");
let excerpt = ImportantExcerpt { part: first_sentence };

// Lifetime elision rules (compiler infers)
// Rule 1: Each parameter gets its own lifetime
// Rule 2: If one input lifetime, assigned to all outputs
// Rule 3: If &self or &mut self, its lifetime assigned to outputs

// Examples that don't need annotations due to elision:
fn first_word(s: &str) -> &str { // inferred: fn first_word<'a>(s: &'a str) -> &'a str
    &s[..1]
}

// Method with lifetimes
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // no annotation needed
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part // self's lifetime returned
    }
}

// Static lifetime - lives for entire program
let s: &'static str = "I have a static lifetime.";

// Multiple lifetimes
fn longest_with_announcement<'a, 'b>(
    x: &'a str,
    y: &'a str,
    ann: &'b str,
) -> &'a str {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

## Closures & Iterators

Closures are anonymous functions that can capture their environment. Iterators provide a lazy way to process sequences of elements.

### Closures
```rust
// Basic closure
let add_one = |x| x + 1;
println!("{}", add_one(5)); // 6

// Type annotations (optional)
let add_one = |x: i32| -> i32 { x + 1 };

// Capturing environment
let x = 4;
let equal_to_x = |z| z == x; // captures x
println!("{}", equal_to_x(4)); // true

// Three ways to capture:
// 1. FnOnce - takes ownership
// 2. FnMut - mutable borrow
// 3. Fn - immutable borrow

// FnOnce example
let s = String::from("hello");
let consume = || {
    println!("{}", s);
    drop(s); // takes ownership
};
consume();
// consume(); // Error: s already moved

// FnMut example
let mut list = vec![1, 2, 3];
let mut mutate = || list.push(4);
mutate();

// Fn example
let list = vec![1, 2, 3];
let borrow = || println!("{:?}", list);
borrow();
borrow(); // can call multiple times

// Move keyword (force ownership)
let list = vec![1, 2, 3];
let print = move || println!("{:?}", list);
print();
// println!("{:?}", list); // Error: list was moved

// Closures as parameters
fn apply<F>(f: F, x: i32) -> i32
    where F: Fn(i32) -> i32
{
    f(x)
}

let double = |x| x * 2;
println!("{}", apply(double, 5)); // 10

// Returning closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

### Iterators
```rust
// Basic iteration
let v = vec![1, 2, 3];

// for loop (implicit iterator)
for val in &v {
    println!("{}", val);
}

// Explicit iterator
let mut iter = v.iter();
assert_eq!(iter.next(), Some(&1));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&3));
assert_eq!(iter.next(), None);

// Three forms:
// iter() - immutable references
// iter_mut() - mutable references
// into_iter() - takes ownership

// Consuming adaptors (consume iterator)
let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum(); // 6

// Iterator adaptors (produce new iterators)
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();

// Common iterator methods
let v = vec![1, 2, 3, 4, 5, 6];

// map - transform
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();

// filter - keep matching
let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();

// filter_map - filter and map together
let doubled_evens: Vec<_> = v.iter()
    .filter_map(|x| {
        if x % 2 == 0 {
            Some(x * 2)
        } else {
            None
        }
    })
    .collect();

// fold/reduce
let sum = v.iter().fold(0, |acc, x| acc + x);

// take - first n elements
let first_three: Vec<_> = v.iter().take(3).collect();

// skip - skip first n elements
let after_two: Vec<_> = v.iter().skip(2).collect();

// zip - combine two iterators
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];
let combined: Vec<_> = a.iter().zip(b.iter()).collect();

// enumerate - index with value
for (i, val) in v.iter().enumerate() {
    println!("{}: {}", i, val);
}

// chain - combine iterators
let a = vec![1, 2, 3];
let b = vec![4, 5, 6];
let chained: Vec<_> = a.iter().chain(b.iter()).collect();

// find - first matching element
let first_even = v.iter().find(|&&x| x % 2 == 0);

// any/all - check conditions
let has_even = v.iter().any(|x| x % 2 == 0);
let all_positive = v.iter().all(|x| *x > 0);

// Chaining multiple operations
let result: Vec<_> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();

// Custom iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

let counter = Counter::new();
for num in counter {
    println!("{}", num); // 1, 2, 3, 4, 5
}
```

---

## Concurrency

Rust's ownership system prevents data races at compile time. Threads, message passing, and shared state are all safe by default.

### Threads
```rust
use std::thread;
use std::time::Duration;

// Spawn a thread
thread::spawn(|| {
    for i in 1..10 {
        println!("Number {} from spawned thread", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// Join handle (wait for thread)
let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("Number {} from spawned thread", i);
    }
});

handle.join().unwrap(); // blocks until thread finishes

// Move closure to transfer ownership
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Vector: {:?}", v);
});

handle.join().unwrap();
// v is no longer valid here
```

### Message Passing (Channels)
```rust
use std::sync::mpsc; // multiple producer, single consumer
use std::thread;

// Create channel
let (tx, rx) = mpsc::channel();

// Send from thread
thread::spawn(move || {
    let val = String::from("hello");
    tx.send(val).unwrap();
    // val is no longer valid (moved)
});

// Receive in main thread
let received = rx.recv().unwrap();
println!("Got: {}", received);

// Sending multiple values
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ];
    
    for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
});

// Receive as iterator
for received in rx {
    println!("Got: {}", received);
}

// Multiple producers
let (tx, rx) = mpsc::channel();
let tx1 = tx.clone(); // clone transmitter

thread::spawn(move || {
    tx.send(String::from("from tx")).unwrap();
});

thread::spawn(move || {
    tx1.send(String::from("from tx1")).unwrap();
});

for received in rx {
    println!("Got: {}", received);
}
```

### Shared State (Mutex, Arc)
```rust
use std::sync::{Mutex, Arc};
use std::thread;

// Mutex - mutual exclusion lock
let m = Mutex::new(5);

{
    let mut num = m.lock().unwrap(); // acquire lock
    *num = 6;
} // lock released when num goes out of scope

// Arc - Atomic Reference Counted (thread-safe)
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap()); // 10

// RefCell - interior mutability (not thread-safe)
// Use Arc<Mutex<T>> for thread-safe interior mutability
```

### Sync and Send Traits
```rust
// Send - ownership can be transferred between threads
// Most types are Send, except Rc<T>

// Sync - safe to reference from multiple threads
// Types are Sync if &T is Send

// Most primitive types are Send + Sync
// Manually implementing Send/Sync is unsafe
```

---

## Modules & Crates

Rust's module system organizes code into reusable units. Crates are packages, and modules organize code within a crate.

### Modules
```rust
// Defining modules
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        
        fn seat_at_table() {} // private
    }
    
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// Using paths
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// super - parent module
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // call parent's function
    }
    
    fn cook_order() {}
}

// Struct fields privacy
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,     // public
        seasonal_fruit: String, // private
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// Enum variants are all public when enum is public
mod back_of_house {
    pub enum Appetizer {
        Soup,  // public
        Salad, // public
    }
}

// use keyword - bring paths into scope
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// Idiomatic use paths
use std::collections::HashMap; // bring function's parent module
use std::fmt::Result;
use std::io::Result as IoResult; // rename to avoid conflicts

// Re-exporting with pub use
pub use crate::front_of_house::hosting;

// Nested paths
use std::{cmp::Ordering, io};
use std::io::{self, Write}; // brings both io and io::Write

// Glob operator
use std::collections::*; // brings all public items
```

### File Organization
```rust
// src/lib.rs or src/main.rs
mod front_of_house; // looks for src/front_of_house.rs or src/front_of_house/mod.rs

// src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}

// Or src/front_of_house/mod.rs with src/front_of_house/hosting.rs
```

### Cargo & Crates
```bash
# Create new binary project
cargo new my_project

# Create new library
cargo new --lib my_library

# Build project
cargo build
cargo build --release

# Run project
cargo run

# Check compilation without building
cargo check

# Run tests
cargo test

# Generate documentation
cargo doc --open

# Add dependencies (edit Cargo.toml)
[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }

# Update dependencies
cargo update
```

### Cargo.toml
```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[profile.release]
opt-level = 3
lto = true # link-time optimization

[[bin]]
name = "my_binary"
path = "src/main.rs"

[lib]
name = "my_library"
path = "src/lib.rs"
```

---

## Testing

Rust has built-in support for unit tests, integration tests, and documentation tests. Tests are written as regular functions with the `#[test]` attribute.
```rust
// Unit tests (in same file as code)
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn another() {
        assert!(true);
        assert_ne!(2, 3);
    }
    
    #[test]
    #[should_panic]
    fn this_should_panic() {
        panic!("Make this test fail");
    }
    
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    // Test with Result
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    
    #[test]
    #[ignore] // ignore by default, run with cargo test -- --ignored
    fn expensive_test() {
        // ...
    }
}

// Custom messages
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}

// Integration tests (in tests/ directory)
// tests/integration_test.rs
use my_library;

#[test]
fn it_adds_two() {
    assert_eq!(4, my_library::add_two(2));
}

// Test organization
// tests/common/mod.rs - common setup code
// tests/integration_test.rs - uses common module

// Running tests
// cargo test - run all tests
// cargo test test_name - run specific test
// cargo test --test integration_test - run specific integration test
// cargo test -- --test-threads=1 - run tests serially
// cargo test -- --show-output - show println! output
// cargo test -- --ignored - run only ignored tests

// Documentation tests
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

---

## Common Macros

Macros generate code at compile time. They're distinguished by the `!` suffix.
```rust
// println! - print to stdout with newline
println!("Hello, world!");
println!("The answer is {}", 42);
println!("{:?}", vec![1, 2, 3]); // debug print
println!("{:#?}", complex_struct); // pretty debug print

// print! - print without newline
print!("Loading");

// eprintln! - print to stderr
eprintln!("Error: {}", error_message);

// format! - create formatted String
let s = format!("x = {}, y = {}", 10, 20);

// vec! - create vector
let v = vec![1, 2, 3];

// panic! - unrecoverable error
panic!("Something went wrong!");

// assert! - runtime assertion
assert!(1 == 1);
assert!(x > 0, "x must be positive, got {}", x);

// assert_eq! / assert_ne!
assert_eq!(result, expected);
assert_ne!(a, b);

// dbg! - debug print and return value
let a = 2;
let b = dbg!(a * 2) + 1; // prints [src/main.rs:2] a * 2 = 4

// matches! - pattern matching expression
let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

// unimplemented! - placeholder
fn not_yet_implemented() {
    unimplemented!("This function is not done yet");
}

// todo! - similar to unimplemented! but for known todos
fn work_in_progress() {
    todo!("Finish this function");
}

// unreachable! - mark code that should never be reached
match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => unreachable!("value should only be 1 or 2"),
}

// include_str! / include_bytes! - include file contents at compile time
let html = include_str!("template.html");
let data = include_bytes!("data.bin");

// env! - get environment variable at compile time
let path: &str = env!("PATH");

// concat! - concatenate literals at compile time
let s = concat!("test", 10, 'b', true);

// Defining simple macros
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

say_hello!();

// Macro with parameters
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Function {:?} was called", stringify!($func_name));
        }
    };
}

create_function!(foo);
foo(); // prints: Function "foo" was called
```

---

## Best Practices

- **Use `cargo clippy`**: Linter that catches common mistakes and suggests improvements
- **Use `cargo fmt`**: Automatic code formatting following Rust style guidelines
- **Prefer iterators over loops**: More idiomatic and often optimized better
- **Use `?` for error propagation**: Cleaner than explicit `match` for errors
- **Avoid `unwrap()` in production**: Use proper error handling with `Result` and `Option`
- **Use `&str` over `String` for function parameters**: More flexible, accepts both
- **Implement `Display` for user-facing output, `Debug` for debugging**: Clear separation of concerns
- **Use `impl Trait` for return types**: Simpler than concrete types when flexibility not needed
- **Prefer newtype pattern for type safety**: Wrap primitive types to prevent mistakes
- **Use builder pattern for complex constructors**: Improves readability and flexibility
- **Use `#[derive]` for common traits**: Automatic implementation for Debug, Clone, etc.
- **Avoid premature optimization**: Write clear code first, optimize based on profiling
- **Use `Cow<str>` for flexible string handling**: Clones only when necessary
- **Prefer `match` over `if let` for complex patterns**: More exhaustive and clear
- **Use type aliases for complex types**: `type Result<T> = std::result::Result<T, MyError>`
- **Document public APIs**: Use `///` for doc comments that generate documentation
- **Use `const` and `const fn` when possible**: Compile-time computation improves performance
- **Avoid large stack allocations**: Use `Box` or heap allocation for large data structures
- **Prefer `&[T]` over `&Vec<T>` for parameters**: More general, accepts slices and arrays
- **Use `Default` trait for default values**: Standard way to create default instances

---

## Common Mistakes

### Borrowing errors
```rust
// ❌ Wrong - cannot borrow as mutable while immutable borrow exists
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s; // Error
println!("{}", r1);

// ✅ Correct - immutable references out of scope before mutable
let mut s = String::from("hello");
let r1 = &s;
println!("{}", r1);
// r1 no longer used
let r2 = &mut s;
```

### Moving values accidentally
```rust
// ❌ Wrong - value moved
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1); // Error: s1 was moved

// ✅ Correct - clone for independent copies
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);

// ✅ Correct - borrow instead
let s1 = String::from("hello");
let s2 = &s1;
println!("{} {}", s1, s2);
```

### Returning references to local variables
```rust
// ❌ Wrong - returning reference to local variable
fn dangle() -> &String {
    let s = String::from("hello");
    &s // s will be dropped
} // Error: missing lifetime specifier

// ✅ Correct - return owned value
fn no_dangle() -> String {
    let s = String::from("hello");
    s // ownership transferred
}
```

### Using `unwrap()` without handling errors
```rust
// ❌ Wrong - panics if file doesn't exist
let contents = std::fs::read_to_string("file.txt").unwrap();

// ✅ Correct - handle error
let contents = match std::fs::read_to_string("file.txt") {
    Ok(c) => c,
    Err(e) => {
        eprintln!("Error reading file: {}", e);
        return;
    }
};

// ✅ Correct - propagate error
fn read_file() -> Result<String, std::io::Error> {
    let contents = std::fs::read_to_string("file.txt")?;
    Ok(contents)
}
```

### Integer overflow in release mode
```rust
// ❌ Wrong - silently wraps in release builds
let mut num: u8 = 255;
num += 1; // becomes 0 in release

// ✅ Correct - use checked operations
let mut num: u8 = 255;
num = num.checked_add(1).unwrap_or(255);

// ✅ Correct - or saturating operations
num = num.saturating_add(1); // stays at 255
```

### Not handling all enum variants
```rust
// ❌ Wrong - not exhaustive
enum Color {
    Red,
    Green,
    Blue,
}

fn describe(color: Color) -> &'static str {
    match color {
        Color::Red => "red",
        Color::Green => "green",
        // Missing Blue - won't compile
    }
}

// ✅ Correct - handle all variants
match color {
    Color::Red => "red",
    Color::Green => "green",
    Color::Blue => "blue",
}
```

### Using `String` when `&str` would work
```rust
// ❌ Wrong - forces allocation
fn greet(name: String) {
    println!("Hello, {}!", name);
}

// ✅ Correct - accepts both String and &str
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

---

## Quick Reference

### Variable Declaration
```rust
let x = 5;              // immutable
let mut x = 5;          // mutable
const MAX: u32 = 100;   // constant
```

### Common Types
```rust
i32, i64, u32, u64      // integers
f32, f64                // floats
bool                    // boolean
char                    // character
&str                    // string slice
String                  // owned string
Vec<T>                  // vector
Option<T>               // Some(T) or None
Result<T, E>            // Ok(T) or Err(E)
```

### Control Flow
```rust
if condition { }
while condition { }
for item in collection { }
loop { }
match value { pattern => expression }
```

### Functions
```rust
fn function_name(param: Type) -> ReturnType { }
```

### Ownership
```rust
let s1 = s;             // move
let s2 = s.clone();     // deep copy
let s3 = &s;            // borrow (immutable)
let s4 = &mut s;        // borrow (mutable)
```

### Common Methods
```rust
.len()                  // length
.is_empty()             // check if empty
.push()                 // add to collection
.pop()                  // remove from collection
.iter()                 // create iterator
.collect()              // collect into collection
.map()                  // transform
.filter()               // filter
.unwrap()               // get value or panic
.expect()               // unwrap with message
```

### Cargo Commands
```bash
cargo new project       # new project
cargo build            # compile
cargo run              # run
cargo test             # test
cargo doc              # documentation
cargo fmt              # format code
cargo clippy           # lint
```
