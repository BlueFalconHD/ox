// oxide language
// concept of syntax

// comments use # or ###...###

// functions are declared by the keyword `fn` followed by the function name
// if you wish to make a function a member of a struct, you use the syntax `fn <struct_name>::<function_name>`
// the arguments are enclosed in parentheses, with a colon syntax to specify types
// the return type is specified after the arguments via the `->` operator
// the body of the function is enclosed in curly braces

// the main function is the entry point of the program
// it will always be run first unless specified otherwise at compile time

fn main() -> Number where size 32 {
    // the `println!` macro is used to print to the console
    // it is similar to the `printf` function in C
    println!("Hello, world!");
}

// functions can be called using the syntax of `<function_name>(<arguments>)`
// if a function was defined as a macro, it can be implicitly called by using the syntax `<function_name>!`

// variables are declared using the `let <variable_name>: (<type info>)` syntax
// if you want to type a variable, you can use the `: <type>` syntax
// if the variable conforms to multiple types or has a special property, you can use parentheses with commas to add multiple types or properties
// a variable is immutable by default, meaning that it cannot be changed after it is declared
// to make a variable mutable, you can add mut to the where syntax
// variables cannot be assigned before they are declared
// variables can be assigned using the `=` operator

let x: Number where size 32 = 5;
let y: String = "Hello, world!";

// a predicate is a statement that evaluates to a boolean value
// predicates include logical comparisons, such as `==`, `!=`, `>`, `<`, `>=`, `<=`, `&&`, `||`, and `!`

let a: Bool = true;
let b: Bool = false;
let abracadabra: Bool = (a && b) != (a || b);

// oxide has no if statements, it only has switch cases which allows for more robust error handling and control flow
// the switch case syntax is `switch <variable> as <name for predicates> { <predicate> => { <code> } }`
// also, switch cases kind of just... look prettier

switch abracadabra as a {
    (a == true) => {
        println!("Abracadabra!");
    }
    (a == false) => {
        println!("No abracadabra!");
    }

    # the default case is required
    default => {
      println!("Abracadabra is not a boolean! wtf");
    }
}

// the where syntax is used to specify additional properties of a type
// for example, a number can have a size of 8, 16, 32, or 64 bits,
// instead of having an individual type for each size, the size is specified using the where syntax
// the size defaults to a sane value (most likely word size) if not specified
// once a type's properties are specified, they cannot be modfied, only extended

let c: Number where size 8 = 0;

// errors are stored as values, not thrown
// this way, errors can be handled in a more robust manner



// a library is oxide code made to be imported into other oxide code
// it provides a way to add functionality to the language without modifying the language itself
// libraries are imported using the `use <library_name>` syntax
// if you have many libraries to import, you can use the `use { <library_name>, ... }` syntax
// if you need to alias an import, you can use the `use <library_name> as <alias>` syntax

use std;

// you can namespace imports by using the `use <library_name>::<function_name>` syntax
// this will import only the specified sub-library or function

use std::io;

// the oxide standard library is minimal and only contains the most basic functions
// this is to keep the language lightweight and allow for a wider range of use cases

// one of the main parts of the standard library is types and utility functions
// when importing std::types, you get access to Float, String, Hashmap, and other more complex types
// you can program without the standard library types, but you might have to create your own logic for certain functions

use std::types;
let d: Float where type IEEE = 3.14;

// the standard library also contains functions for interacting with the operating system
// std::file contains functions for reading and writing to the filesystem
// std::process contains functions for running other programs, managing processes, and interacting with the environment
// std::net contains functions for networking and interacting with the internet
// std::os contains functions for interacting with the operating system itself, such as environment variables and system information
// std::time contains functions for working with time and dates
// std::math contains functions for mathematical operations other than basic arithmetic, like sin, cos, and log
// std::rand contains functions for generating random numbers
// std::collections contains functions for working with collections of data, like arrays, lists, and maps
// std::io contains functions for input and output, like reading from the console and writing to files
// std::error contains functions for working with errors and exceptions

// print "Hello, world!" to the console
use std::io;
io::println! "Hello, world!";

// open "file.txt" and write "Hello, file!" to it
use std::file;
let fhandle: file::FileHandle = file::open("file.txt", file::READ_WRITE)
switch FileHandle {
    (Ok) => {
        fhandle.write("Hello, file!");
        fhandle.close!
    }
    (Err) => {
        io::println! "Error opening file!";
    }
}

// run "ls" and print the output to the console
use std::process;
let proc: process::Process = process::run("ls")
let output: String = proc -> output;
io::println! output;

// get the current time and print it to the console
use std::time;
// we want to use the time format of hours:minutes:seconds
let t: time::Time = time::now();
io::println! t -> format("%H:%M:%S");

// calculate the sine of 3.14 and print it to the console
use std::math;
let result: Float where type IEEE = math::sin(3.14);
io::println! result;

// print the result of GET request to "http://example.com"
use std::net;
let response: String = net::get("http://example.com");
io::println! response -> status;

// get the value of the environment variable "PATH" and print it to the console
use std::os;
let path: String = os::getenv("PATH");
io::println! path;

// generate a random number between 1 and 100 and print it to the console using the current unix time as a seed
use std::rand;
let seed: Number where size 64 = time::now() -> unix;
let gen: rand::Generator = rand::new(seed);
let num: Number where size 32 = gen -> int(1, 100);
io::println! num;

// create a list of numbers from 1 to 10 and print it to the console
use std::collections;
let map: collections::Map<Number, Number> = collections::Map();
range x (1, 10) -> map.insert(x, x - 1) ;
io::println! map;

// read a line from the console and print it to the console
let input: String = io::read_line();
io::println! input;



// A struct is a user-defined type that contains a collection of named fields
// Structs are used to group related data together, organize code, and create custom types
// Structs are defined using the `struct` keyword followed by the name of the struct and a list of fields

struct Point {
    x: Number,
    y: Number,
}

// Structs can have methods, which are functions that operate on the struct's data
// Methods are defined in a struct's body using the `fn` keyword followed by the method name and parameters

struct Point {
    x: Number,
    y: Number,

    fn new(x: Number, y: Number) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, other: &Point) -> Number {
        let dx: Number = self.x - other.x;
        let dy: Number = self.y - other.y;
        (dx * dx + dy * dy) -> sqrt
    }
}

// Structs can be instantiated using the `struct_name::new` syntax
// The `::new` function is a special method that creates a new instance of the struct
// It must be defined in every struct's body

let p1: Point = Point::new(0, 0);
let p2: Point = Point::new(3, 4);

// Struct fields can be accessed using the -> operator

io::println! p1 -> x;
io::println! p1 -> y;

// Struct methods are invoked using the -> operator too

let dist: Number = p1 -> distance(&p2);
io::println! dist;

// Structs can also implement features to integrate into the rest of the language
// The following methods can be implemented for a struct for additional functionality:
// - _add: for adding two instances of the struct
// - _subtract: for subtracting two instances of the struct
// - _multiply: for multiplying two instances of the struct
// - _divide: for dividing two instances of the struct
// - _equal: for checking if two instances of the struct are equal
// - _not_equal: for checking if two instances of the struct are not equal
// - _less_than: for checking if one instance of the struct is less than another
// - _greater_than: for checking if one instance of the struct is greater than another
// - _less_than_or_equal: for checking if one instance of the struct is less than or equal to another
// - _greater_than_or_equal: for checking if one instance of the struct is greater than or equal to another
// - _iter: for iterating over an instance of the struct
// - _index: for indexing into an instance of the struct
// - _len: for getting the length of an instance of the struct
// - _cast: for casting an instance of the struct to another type
// - _hash: for hashing an instance of the struct
// - _debug: for displaying debug information about an instance of the struct
// - _get: for getting a field of an instance of the struct
// - _set: for setting a field of an instance of the struct
// - _call: for calling an instance of the struct as a function
// - _drop: for cleaning up an instance of the struct
// - _clone: for creating a copy of an instance of the struct
// - _get_value: for getting the value of an instance of the struct, eg. for a number or string (this is for a struct that is made to be a type, like a Number, which shouldn't be treated as a struct, but as a value)
// - _set_value: for setting the value of an instance of the struct, eg. for a number or string (this is for a struct that is made to be a type, like a Number, which shouldn't be treated as a struct, but as a value)
