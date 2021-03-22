fn main() {
   let s1 = String::from("hello");
   attempt_mutate_on_immutable(s1);
   //println!("should be hello: {}",s1); // should error, borrowed

   let mut s2 = String::from("hello");
   attempt_mutate_on_mutable(&mut s2);
   println!("should be hello, again: {}",s2);
}

fn attempt_mutate_on_immutable(in_string: String) {
	//in_string.push_str(", again"); // should error, not mutable
	println!("should be hello: {}", in_string);
}

fn attempt_mutate_on_mutable(in_string: &mut String) {
	in_string.push_str(", again");
	println!("should be hello, again: {}", in_string);
}

