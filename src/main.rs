
use std::io;
extern crate rand;

fn main(){
	let mut var = "my var";
	println!("Hello, world! {}", var);
	getSrtingSrez("String srez");
	getDynamicSrting("String dynamic".to_string());
}


fn getSrtingSrez( str : &str ){
	println!("getSrtingSrez = {}", str);
}

fn getDynamicSrting( str : String ){
	println!("getSrtingSrez = {}", str);
}


