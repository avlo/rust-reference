fn main() {
	let mut s = "string";
	println!("{}", s);
	s = "another";
	println!("{}", s);

	//let s1 = String::from("hello");
	//let s2 = s1;
	//println!("{}", s2);

	let s1 = String::from("hello");
	let s2 = s1;
	println!("{}, world!", s1);
}
