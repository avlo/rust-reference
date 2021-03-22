fn main() {
	let s = String::from("hello");
	takes_ownership(s); // s moved
	// println!("{}", s); // will not work, s has been moved

	let x = 5;
	makes_copy(x); // x is primitive type, copied to stack, so not moved
	println!("{}", x);
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}
