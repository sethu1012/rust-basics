use std::cell::RefCell;
#[allow(dead_code)]
#[allow(unused_variables)]
// #[allow()]
extern crate rand;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::io::stdin;
use std::iter::Sum;
use std::mem;
use std::ops::{Add, AddAssign, Neg};
use std::rc::Rc;

use rand::Rng;

mod calc;

// {:?} - used for debugging

const SOME_CONSTANT: i8 = 0; // constant

static Z: i8 = 0; // gets the address of the variable
// using mutable static variables is not advisable

static mut ZM: i8 = 0; // gets the address of the variable
// using mutable static variables is not advisable

#[derive(Debug)] // Derives all the traits of Debug
struct Point {
    x: i64,
    y: i64,
}

struct PointWithGenerics<T> {
    x: T,
    y: T,
}

struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn len(&self) -> f64 {
        let dx = (self.p1.x - self.p2.x) as f64;
        let dy = (self.p1.y - self.p2.y) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

enum Color {
    Red,
    Green,
    Blue,
    RGBColor(u8, u8, u8),
    CMYK {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
} // RGBColor is a tuple,, CMYK is a struct

union IntOrFloats {
    i1: i32,
    f1: f32,
}

// 1st of 3 ways
// Traits as parameters
fn print_info(point: impl Debug) { // impl Debug is mandatory for custom structs
    println!("{:?}", point);
}

// 2nd of 3 ways
// Traits as parameters
fn print_info2<T: Debug>(point: T) { // use '+' to have multiple traits
    // eg. <T: Shape + Debug> -> Shape is a custom trait
    println!("{:?}", point);
}

// 3rd of 3 ways
// Traits as parameters
fn print_info3<T>(point: T) where T: Debug { // use '+' to have multiple traits
    // eg. <T: Shape + Debug> -> Shape is a custom trait
    println!("{:?}", point);
}

fn get_point() {
    let p1: Point = Point { x: 4, y: 5 }; // structs
    println!("Point: ({}, {})", p1.x, p1.y);
    print_info(&p1);
    print_info2(&p1);
    let p2: Point = Point { x: 5, y: 6 };
    println!("Point: ({}, {})", p2.x, p2.y);
    print_info(&p2);
    print_info2(&p2);
    let line: Line = Line { p1, p2 }; // Nested structs
    println!("Line is at {}", line.p1.x);
    println!("Length of line: {}", line.len())
}

fn get_color() { // enums
    let c: Color = Color::Blue;
    // let c: Color = Color::RGBColor(0, 0, 0);
    // let c: Color = Color::CMYK { cyan: 0, magenta: 0, yellow: 0, black: 0 };
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGBColor(0, 0, 0) => println!("Black"),
        Color::CMYK { cyan: _, magenta: _, yellow: _, black: _ } => println!("Every CMYK Color"),
        // _ represents all values
        _ => println!("Some other color")
    }
}

fn get_union() {
    let mut iof = IntOrFloats { f1: 0.0 };
    // iof.i1 = 42;
    iof.f1 = 42.0;

    let value = unsafe { iof.i1 }; // rust makes it mandatory for us to
    // unsafe when dealing with unions.
    println!("Value = {}", value);
    process_value(iof);
}

fn process_value(iof: IntOrFloats) {
    unsafe {
        match iof {
            IntOrFloats { i1: 42 } => println!("Some Int Value"), // matches the exact value in union
            IntOrFloats { f1 } => println!("{}", f1) // gets treated like
            // a floating point number
        }
    }
}

fn option_usage() { // can return one of two values: Some or None
    let x: f64 = 3.0;
    let y: f64 = 1.0;

    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("Result: {}", z),
        None => println!("Cannot divide by 0"),
    }

    if let Some(z) = result { // alternative to the above statement without none
        println!("Result: {}", z)
    }
}

fn array_usage() {
    // let mut a: [i32;5] = [1, 2, 3, 4, 5]; // array with 5 elements
    let mut a = [1, 2, 3, 4, 5]; // also works
    println!("Len = {}, First = {}", a.len(), a[0]);
    a[0] = 321;
    println!("Len = {}, First = {}", a.len(), a[0]);

    println!("{:?}", a); // Debug

    if a != [1, 2, 3, 4, 5] {
        println!("Different Arrays");
    }

    // let b = [1; 10]; // Fill 10 1s
    let b = [1u16; 10]; // Fill 10 1s - 16 bits
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    let matrix: [[f32; 3]; 2] = [ // matrix
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", matrix);

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if i == j {
                println!("{}", matrix[i][j]);
            }
        }
    }
}

fn slices_usage() {
    let mut a = [1, 2, 3, 4, 5];
    // let mut data= &mut a[1..4]; // slices
    let mut data = &mut a; // slices
    println!("{:?}", data);
    println!("Slice: {}, First Element: {}", data.len(), data[0]);
}

fn tuples_usage(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}

fn pattern_matching() {
    for x in 0..13 {
        match x {
            0 => println!("I have no oranges"),
            1 | 2 => println!("I have one or two oranges"),
            12 => println!("I have a dozen oranges"),
            9...11 => println!("I have lots of oranges"),
            _ if x % 2 == 0 => println!("I have some oranges"),
            _ => println!("I have a few oranges"),
        }
    }

    // let point = (3, 4);
    let point = (0, 0);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("{} {}", x, y)
    }

    let c: Color = Color::Blue;
    // let c: Color = Color::RGBColor(0, 0, 0);
    // let c: Color = Color::CMYK { cyan: 0, magenta: 0, yellow: 0, black: 0 };
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RGBColor(0, 0, 0) => println!("Black"),
        Color::CMYK { black: _, .. } => println!("Every CMYK Color"), // only concerned about black
        // other values don't matter (.. is more like the spread operator)
        // _ represents all values
        _ => println!("Some other color")
    }
}

fn generics() {
    // let a: PointWithGenerics<i64> = PointWithGenerics {x: 0, y: 4};
    // let b = PointWithGenerics {x: 0.0, y: 4.0}; // T<> is optional
}

fn vector_usage() {
    let mut a: Vec<i64> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("{:?}", a);
    // type of index value is usize!!!
    // println!("First Element: {}", a[0]); // Works
    match a.get(4) { // get returns Some or None. Match against the
        // values for further processing.
        Some(x) => println!("{}", x),
        None => println!("No such element")
    }

    for x in &a { println!("{}", x); };

    let b = a.pop(); // removes the last element
    match b {
        Some(x) => println!("{} {:?}", x, a),
        None => println!("Vector is empty"),
    }

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn hashmap_usage() {
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("circle"), 0);

    println!("Square has {} sides", shapes["square".into()]);

    for (key, value) in &shapes {
        println!("{} = {}", key, value);
    }

    shapes.insert("square".into(), 5); // overwrites existing values
    println!("{:?}", shapes);

    shapes.entry("pentagon".into()).or_insert(5); // insert into hm if the key doesn't exist
    println!("{:?}", shapes);

    {
        let actual = shapes.entry("circle".into()).or_insert(0);
        *actual = 4; // change the content of the actual hm
        println!("{:?}", actual);
    }
    println!("{:?}", shapes);
}

fn hashset_usage() {
    let mut greeks = HashSet::new();
    greeks.insert("alpha");
    greeks.insert("beta");
    greeks.insert("gamma");
    greeks.insert("delta");
    greeks.insert("omega");

    println!("{:?}", greeks); // unordered collection

    let is_epsilon_added = greeks.insert("epsilon"); // returns true
    if is_epsilon_added {
        println!("Epsilon was added. New Set: {:?}", greeks);
    }

    if !greeks.contains("kappa") {
        println!("Kappa is missing. Set: {:?}", greeks);
    }

    let removed = greeks.remove("epsilon");
    if removed {
        println!("Epsilon was removed. Set: {:?}", greeks);
    }

    let range_1_5: HashSet<_> = (1..=5).collect();
    let range_6_10: HashSet<_> = (6..=10).collect();
    let range_1_10: HashSet<_> = (1..=10).collect();
    let range_2_8: HashSet<_> = (2..=8).collect();

    // subset
    println!("is {:?} a subset of {:?}? {}",
             range_2_8, range_1_10, range_2_8.is_subset(&range_1_10));

    // no common elements
    println!("is {:?} disjoint of {:?}? {}",
             range_1_5, range_6_10, range_1_5.is_disjoint(&range_6_10));

    println!("{:?} union {:?}? {:?}",
             range_2_8, range_6_10, range_2_8.union(&range_6_10));
    println!("{:?} intersection {:?}? {:?}",
             range_1_5, range_6_10, range_1_5.intersection(&range_6_10));

    println!("{:?} difference {:?}? {:?}",
             range_2_8, range_6_10, range_2_8.difference(&range_6_10));

    // union - intersection = sd
    println!("{:?} symmetric difference {:?}? {:?}",
             range_1_5, range_6_10, range_1_5.symmetric_difference(&range_6_10));
}

fn iterator_usage() {
    let vec = vec![3, 2, 1];
    let mut vec1 = vec![3, 2, 1];
    let mut vec2 = vec![4, 5, 6];
    let mut vec3 = vec![7, 8, 9];


    for x in vec.iter() {
        println!("{}", x);
    }

    for x in vec.iter().rev() {
        println!("{}", x);
    }

    for x in vec1.iter_mut() {
        *x *= *x;
        println!("{}", x);
    }

    vec3.extend(vec2); // extend uses into_iter method
    println!("{:?}", vec3);
}

fn string_usage() {
    // static string
    let s: &'static str = "Hello there"; // string slice - static reference
    for c in s.chars().rev() {
        println!("{}", c);
    }
    if let Some(fc) = s.chars().nth(0) {
        println!("{}", fc);
    }

    // heap allocated construct
    // String
    let mut letters = String::new();
    let mut a = 'a' as u8;
    let z = 'z' as u8;
    while a <= z {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("Letters: {}", letters);

    // &'static str to String
    // let str: &'static str = &letters;

    // concatenation
    // String + str
    // let z = letters + &letters; // join strings
    // let mut abc = String::from("Hello World");
    let mut abc = "Hello World".to_string(); // both works

    let name = "Something";
    let greeting = format!("Hi, {}", name);
    println!("{0} {1} {0}", abc, greeting);
    println!("{xx} {xy} {xz}", xx = 'a', xy = 'b', xz = 'c');
}

fn number_guessing() {
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let n = input.trim_end().parse::<i64>();
                match n {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("Guess is OOR");
                        } else if guess < number {
                            println!("Guess is low");
                        } else if guess > number {
                            println!("Guess is high");
                        } else {
                            println!("Correct");
                            break;
                        }
                    }
                    Err(e) => println!("{}", e),
                }
            }
            Err(_) => continue,
        }
    }
}

fn closure() {
    let plus_one = |x: i32| -> i32 { x + 1 }; // more like a function
    let a: i32 = 6;
    println!("{}", plus_one(a));

    let plus_two = |x| -> i32 { x + 2 }; // more like a function
    let mut z: i32 = 6;
    println!("{}", plus_two(z));

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f); // changes the value of x
    println!("{}", f);
}

fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn hoc(limit: u32) {
    let sum = (0..)
        .map(|x| x * x)
        .take_while(|&x| x < limit)// condition
        .filter(|x: &u32| is_even(*x))
        .fold(0, |sum, x| sum + x); // pair wise operation

    println!("Sum = {}", sum);
}

// traits - start
trait Animal {
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
    fn create(name: &'static str) -> Self; // this is a static fn because it does not talk self as an argument
}

struct Human {
    name: &'static str,
}

impl Animal for Human {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name);
    }

    fn create(name: &'static str) -> Human {
        Human { name }
    }
}

struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name);
    }

    fn create(name: &'static str) -> Cat {
        Cat { name }
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result = 0;
        for x in self { result += *x }
        return result;
    }
}

struct Person {
    name: String,
}

impl Person {
    // 1st
    // fn new(name: &str) -> Person {
    //     Person { name: name.to_string() }
    // }

    // 2nd
    // fn new<S: Into<String>>(name: S) -> Person {

    // 3rd
    fn new<S>(name: S) -> Person where S: Into<String> {
        Person { name: name.into() }
    }
}

struct Company<'z> { // ' followed by any name
    name: String,
    ceo: &'z Person
}

fn lifetime() {
    let boss = Person { name: String::from("Elon Musk") };
    let company = Company { name: String::from("Tesla"), ceo: &boss };
}

struct Creature {
    name: String,
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters", name);
        Creature { name: name.into() }
    }
}

// Drop is a destructor
impl Drop for Creature {
    // Drop is called at the end of scope
    fn drop(&mut self) {
        println!("{} is no more", self.name);
    }
}

fn traits() {
    let h = Human { name: "John Doe" };
    h.talk(); // if talk is not implemented in Animal for Human, this would've
    // returned "John Doe cannot talk"

    let c = Cat { name: "Ket" };
    c.talk();

    let hc = Human::create("John Doe");
    hc.talk(); // if talk is not implemented in Animal for Human, this would've
    // returned "John Doe cannot talk"

    let cc = Cat::create("Ket");
    cc.talk();

    let hg: Human = Animal::create("John Doe");
    hg.talk(); // if talk is not implemented in Animal for Human, this would've
    // returned "John Doe cannot talk"

    let cg: Cat = Animal::create("Ket");
    cg.talk();

    let a = vec![1, 2, 3];
    println!("Sum = {}", a.sum()); // sum() is implemented for vec in Summable trait

    let john = Person::new("John Doe"); // John Doe is &'static str

    let name: String = "Jane".to_string();
    let jane = Person::new(name/*.as_ref()*/); // this is tedious
    // since we are converting a String to &str (refer 1st new block in impl)
    // 2nd new block takes a generic type and applies conversion wherever possible
    // as_ref is redundant in 2nd new block

    let g1 = Creature::new("G1");
    let ra1 = Creature::new("RA1");

    // Drop cannot be called explicitly with a variable
    // g1.drop() doesn't work;
    // However, there's a workaround
    drop(g1); // Destroyed
    // drop() is almost unnecessary
}
// traits - end

// operator overloading - starts
#[derive(Debug, PartialEq, Eq)]
struct Complex<T> {
    r: T,
    i: T,
}

impl<T> Complex<T> {
    fn new(r: T, i: T) -> Complex<T> {
        Complex::<T> { r, i } // without :: rust compiler will treat '<' as less than symbol
    }
}

// this is specifically for i32
// impl Add for Complex<i32> {
//     type Output = Complex<i32>;
//
//     // support op a + b
//     fn add(self, rhs: Self) -> Self::Output { // Self is for Complex<i32>
//         Complex{
//             r: self.r + rhs.r,
//             i: self.i + rhs.i
//         }
//     }
// }

// for any type
impl<T> Add for Complex<T> where T: Add<Output=T> {
    type Output = Complex<T>;

    // support op a + b
    fn add(self, rhs: Self) -> Self::Output { // Self is for Complex<i32>
        Complex {
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}


// for any type
impl<T> AddAssign for Complex<T> where T: AddAssign<T> {
    // support op a += b
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl<T> Neg for Complex<T> where T: Neg<Output=T> {
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            r: -self.r,
            i: -self.i,
        }
    }
}

// impl <T> PartialEq for Complex<T> where T: PartialEq {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.r == rhs.r && self.i == rhs.i
//     }
// }
//
// impl<T> PartialEq<Self> for Complex<T> where T: Eq {
//     fn eq(&self, other: &Self) -> bool {
//         todo!()
//     }
// }

// impl <T> Eq for Complex<T> where T: Eq {}

fn operator_overloading() {
    let mut a = Complex::new(1, 2);
    let mut b = Complex::new(1, 2);

    println!("{:?}", a);
    println!("{:?}", b);
    a += b;
    // println!("{:?}", a.add(b));
    println!("{:?}", a == a);
    println!("{:?}", a);
    println!("{:?}", -a);
}
// operator overloading - end

// static dispatch - start
trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// static
fn print_it<T: Printable>(z: T) {
    println!("{}", z.format());
}

// dynamic
fn print_it_dynamic(z: &Printable) {
    println!("{}", z.format());
}

fn static_dispatch() {
    let a = 123;
    let b = "hello".to_string();

    println!("{}", a.format());
    println!("{}", b.format());

    // print_it(*c); // type is erased when the function is called
    // print_it(*d);
    print_it_dynamic(&a);
    print_it_dynamic(&b);
}
// static dispatch - end

fn if_statement(c: char) {
    if c == 'A' {
        println!("if_statement: c = {}", c);
    } else if c == 'B' {
        println!("if_statement: c = {}", c);
    } else {
        println!("if_statement: c = {}", c);
    }

    let d: char = if c == 'A' { 'A' } else if c == 'B' { 'B' } else { c }; // Same as above
    println!("if_statement: d = {}", d);
}

enum Creatures {
    Human(Human),
    Cat(Cat),
}

fn vectors_of_different_objects() {
    let mut creatures = Vec::new();
    creatures.push(Creatures::Human(
        Human { name: "John" }
    ));
    creatures.push(Creatures::Cat(
        Cat { name: "Ket" }
    ));

    for c in creatures {
        match c {
            Creatures::Human(h) => h.talk(),
            Creatures::Cat(c) => c.talk(),
        }
    }

    // let mut animals: Vec<Box<Animal>> = Vec::new();
    // animals.push(
    //     Box::new(Human { name: "John" })
    // );
    // animals.push(
    //     Box::new(Cat { name: "Ket" })
    // );
    // for a in animals.iter() {
    //     a.talk();
    // }
}

fn ownership() {
    let v = vec![1, 2, 3]; // v owns the vector
    // only a single variable own a particular data
    // let v2 = v; // v is invalidated at this point
    // println!("{}", v); // will now cause an error

    // let foo = |v: Vec<i32>| ();
    // foo(v); // cannot be used
    // println!("{:?}", v);

    // values are copied in case of primitive types
    let u = 1;
    let u2 = u;
    println!("u = {}", u);

    // if u is boxed, it will throw error
    // let u = Box::new(1);
    // let u2 = u;
    // println!("u = {}", u);

    let print_vec = |x: Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    let mod_vec = print_vec(v);
    println!("{:?}", mod_vec);
}

fn borrowing() {}

fn for_statement() {
    for i in 1..11 {
        if i % 2 == 0 { continue; }
        print!("{}, ", i);
    }
    println!();

    for (pos, y) in (30..41).enumerate() {
        print!("{}: {}\n", pos, y);
    }
    println!();
}

fn while_statement() {
    let mut x: u64 = 0;
    while x < 10 {
        print!("{}, ", x);
        x += 1;
    }

    println!();

    loop { // while true
        print!("{}, ", x);
        x -= 1;

        if x < 1 { break; }
    }
    println!();
}

fn match_statement() { // switch statement
    let country_code: i32 = 44;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("Country: {} with country code {}", country, country_code);
}

struct Student {
    name: String,
    courses: Vec<Rc<RefCell<Course>>>
}

impl Student {
    fn new(name: &str) -> Student {
        Student {
            name: name.into(),
            courses: Vec::new()
        }
    }
}

struct Course {
    name: String,
    students: Vec<Rc<RefCell<Student>>>
}

impl Course {
    fn new(name: &str) -> Course {
        Course{
            name: name.into(),
            students: Vec::new()
        }
    }

    fn add_student(
        course: Rc<RefCell<Course>>,
        student: Rc<RefCell<Student>>
    ) {
        student.borrow_mut().courses.push(course.clone());
        course.borrow_mut().students.push(student);
    }
}

fn circular_references() {
    let john = Rc::new(
        RefCell::new(
            Student::new("John")
        )
    );
    let course = Rc::new(
        RefCell::new(
            Course::new("Rust")
        )
    );

    Course::add_student(course, john);
}

fn main() {
    let a: u8 = 32;
    println!("a = {}", a); // immutable - cannot be changed

    let a: u8 = 123; // redeclared - value is overwritten
    println!("a = {}", a); // immutable - cannot be changed

    let mut b: i8 = -128; // i8 is signed. -128 to 127
    b = b + 4;
    println!("b = {}", b);

    let mut c: i32 = 123456789; // signed -2^32 to 2^32 - 1
    println!("c = {} and takes {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {} and takes {} bytes", c, mem::size_of_val(&c));

    let d: isize = 4567; // isize & usize - native to a specific processor
    println!("d = {}", mem::size_of_val(&d));

    let e: char = 'x'; // char data type
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    let f: f32 = 4.5678; // floating point
    println!("f = {}, size = {}", f, mem::size_of_val(&f));

    let g: bool = false; // boolean
    println!("g = {}, size = {}", g, mem::size_of_val(&g));

    println!("SOME_CONSTANT = {}", SOME_CONSTANT);

    println!("static Z = {}", Z);

    unsafe { // to use static mut variables, one must enclose it within the static block
        println!("static mut Z = {}", ZM);
    }

    let h: i32 = calc::add(3, 5);
    println!("Calc: {}", h);

    println!("{}", "A" == "A"); // string equals

    if_statement('C');
    while_statement();
    for_statement();
    match_statement();

    // let mut input: String = String::new();
    // stdin().read_line(&mut input); // that's how to take input in rust
    // println!("Input: {}", input);

    // loop {
    //     let mut input: String = String::new();
    //     match stdin().read_line(&mut input) { // matches ok and error cases
    //         Ok(_) => {
    //             match input {
    //                 _ => {
    //                     println!("Something: {}", input);
    //                     break;
    //                 }
    //             }
    //         },
    //         Err(_) => continue,
    //     };
    // }

    get_point();
    get_color();
    get_union();
    option_usage();
    array_usage();
    slices_usage();
    let tuple = tuples_usage(5, 6);
    println!("{:?}", tuple);
    println!("{0} {1}", tuple.0, tuple.1);
    pattern_matching();
    generics();
    vector_usage();
    hashmap_usage();
    hashset_usage();
    iterator_usage();
    string_usage();

    // number_guessing();
    closure();
    hoc(500);
    traits();
    operator_overloading();
    static_dispatch();
    vectors_of_different_objects();
    ownership();
    borrowing();

    // Rc is not a threadsafe way of passing objects
    // Arc is threadsafe
    circular_references();
}
