use gray_code::gray_code;

fn main() {
	for i in 0..10 {
    	println!("binaire normal = {} | binaire gray = {}", i, gray_code(i));
	}
}
