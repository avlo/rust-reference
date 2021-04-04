~~~~~
variable immutability

- variables are immutable by default
	let x = 5;
	x = 6; // compiler error

- make variable mutable 
	let mut x = 5;
	x = 6; // permitted

~~~~~
shadowing

	let x = 5;
	let x = x + 1; // permissted

why use "let" over "mut"?
	because "let" allows rassign to happen only once

~~~~~
scalar types - stored on stack

- integer
	Length	Signed	Unsigned
	------	------	--------
	8-bit		i8			u8
	16-bit	i16			u16
	32-bit	i32			u32
	64-bit	i64			u64
	128-bit	i128		u128
	arch		isize		usize

- integer literals
	literal		Example
	-------		-------
	Decimal		98_222
	Hex				0xff
	Octal			0o77
	Binary		0b1111_0000

- floating point
	f32		single-precision float
	f64		double-precision float

- boolean
	let t = true;  // implicit type
	let f: bool = true; // explicit type annotation (not necessary)

- character
	let c = 'a';

~~~~~
compound types - stored on stack

- tuple
	grouping together values with a variety of types into one compound type.  fixed length & once declared, cannot grow/shrink
		let tup: (i32, f64, u8) = (500, 6.4, 1);

- array
	when fixed number of elements is desired
	let a = [1, 2, 3, 4, 5];	
	let months = ["January", "February", "March", "April", "May", "June"];

	array with type and size spec
		let a: [i32; 5] = [1, 2, 3, 4, 5];

	alternate initialization
		let a = [3; 5];  // 5 elements all set to value 3

	accessing array elements
		let first = a[0];

~~~~~
functions

	fn main() {
   	another_function();
   	yet_another_function(5, 6);
		
    let x = five(); // function w/ return value
    let x = six(); // function w/ return value
	}

	fn another_function() {
   	println!("Another function.");
	}

	fn yet_another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
	}

	// function w/ return values
	fn five() -> i32 {
    5 // implicit return 5
	}

	// function w/ return values
	fn six() -> i32 {
    return 6; // explicit return 6
	}

~~~~~
conditionals
- if/else if/else must evaluate to boolean

	if number % 4 == 0 {
		println!("number is divisible by 4");
  } else if number % 3 == 0 {
		println!("number is divisible by 2");
  } else {
		println!("number is not divisible by 4, 3, or 2");
	}

~~~~~
iterators

	let a = [10, 20, 30, 40, 50];

	// forward iterator
	for element in a.iter() {
		println!("the value is: {}", element);
	}

	// reverse iterator
	for element in (1..=5).rev() {
		println!("the value is: {}", element);
	}

~~~~~
ownership
	guarantees safe memory w/o GCollector.  managed through ownership system via rule set checked at compile time
	keeping track of what parts of code are using what data on the heap
	minimizing the amount of duplicate data on the heap (no deep copying)
	cleaning up unused data on the heap

rules
  Each value in Rust has a variable that’s called its owner.
  There can only be one owner at a time.
  When the owner goes out of scope, the value will be dropped.

  let s1 = String::from("hello"); // s1 is owner
  let s2 = s1; // s1 ownership "moved" to s2
  println!("{}, world!", s2);  // valid since s2 now owns the string
  println!("{}, world!", s1);  // invalid/exception since s1 no longer owns the string

- stack
	all stack data must have a known, fixed size
	faster than heap since function parameters and local vars are pushed to top of stack, then popped when done

- heap 
	Data with unknown size at compile time or a size that might change must be stored on the heap
	heap is less organized, slower than stack

cloning
	if deep copy is desired, then clone:

  let s1 = String::from("hello"); // s1 is owner
  let s2 = s1.clone(); // deep copy 
  println!("{}, world!", s1);  // valid since cloned & not moved
  println!("{}, world!", s2);  // valid since it's a clone

~~~~~
borrowing

	having references as function parameters is called "borrowing"
	trying to modify a borrowed parameter yields error:

		fn main() {
			let s = String::from("hello");
			change(&s);  // parameter passed by reference is "borrowed" parameter
		}

		fn change(some_string: &String) {  // "borrowed" parameter 
			some_string.push_str("extra"); // trying to modify "borrowed" parameter throws error
		}

	above can be fixed by making the reference parameter 'mutable'

		fn main() {
			let mut s = String::from("hello");
			change(&mut s);  // parameter passed by reference and mutable 
		}

		fn change(some_string: &mut String) { // "borrowed" & "mutable" parameter
			some_string.push_str(", world"); // modifying "borrowed" & "mutable" parameter is ok
		}
	
~~~~~
strings (as related to stack / heap)

- string literals, known ahead of time go on stack
	let s = "hello"; // stack value dropped when out of (local) scope

- String type
	let s = String::from("hello"); // :: namespace operatok


