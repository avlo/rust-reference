fn main() {
   let mut no:i32 = 5;
   mutate_no_to_zero_val(no);
   println!("should be 5: {}",no);
   
   no = 6;
   mutate_no_to_zero_ref(&mut no);
   println!("should be 0: {}",no);
}

fn mutate_no_to_zero_val(mut param_no: i32){
   println!("should be 5: {}",param_no);
   param_no = 0; // assign locally
   println!("should be 0: {}",param_no);
}

fn mutate_no_to_zero_ref(param_no:&mut i32){
   println!("should be 6: {}",param_no);
   *param_no = 0; //de reference
   println!("should be 0: {}",param_no);
}
