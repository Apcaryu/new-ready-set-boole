use adder::adder;

fn main() {
	let a = adder(2, 2);
	let b = adder(28, 8);
	// let c = adder(std::u32::MAX, 10);
	println!("-----------------------------------------");
	println!("a = {} | b = {}", a, b);

	let mut res = 4;
	for i in 1..=10 {
		res = adder(res, 8);
		println!("res[{}] : {} | b{:b}", i, res, res);
	}
}