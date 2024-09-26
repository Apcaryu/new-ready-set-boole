#[derive(Debug, PartialEq)]
struct Nbstack {
	nb_list: u64,
	pub counter: u8,
}

impl Nbstack {
	fn new() -> Nbstack {
		Nbstack { nb_list: 0, counter: 0 }
	}

	fn add_number(&mut self, val: u64) {
		if self.counter == 64 {
			panic!("buffer limit reached")
		}
		match val {
			0 => { 
				self.nb_list = self.nb_list << 1;
				self.counter += 1;
			},
			1 => {
				self.nb_list = (self.nb_list << 1) | 1;
				self.counter += 1; 
			},
			_ => {
				panic!("val input is not a '0' or a '1', val = {}", val);
			},
		}
	}

	fn extract_number(&mut self) -> u64 {
		if self.counter == 0 {
			panic!("stack of number is empty");
		}

		let res = self.nb_list & 1;
		
		self.nb_list >>= 1;
		self.counter -= 1;
		
		res
	}
}

fn negation(stack: &mut Nbstack) {
	let mut tmp = stack.extract_number();

	tmp = tmp ^ 1;
	stack.add_number(tmp);	
}

fn and_operator(stack: &mut Nbstack) {
	let (a, b) = (stack.extract_number(), stack.extract_number());
	stack.add_number(a & b);
}

fn or_operator(stack: &mut Nbstack) {
	let (a, b) = (stack.extract_number(), stack.extract_number());
	stack.add_number(a | b);
}

fn xor_operator(stack: &mut Nbstack) {
	let (a, b) = (stack.extract_number(), stack.extract_number());
	stack.add_number(a ^ b);
}

fn material_condition_operator(stack: &mut Nbstack) {
	let (a, b) = (stack.extract_number(), stack.extract_number());
	println!("a = {} | b = {}", a, b);
	stack.add_number((a ^ 1) | b);
}
pub fn eval_formula(formula: &str) -> bool {
	let mut stack= Nbstack::new();
	
	for chr in formula.chars(){
		match chr {
			'0' => {
				stack.add_number(0);
			},
			'1' => {
				stack.add_number(1);
			},
			'!' => {
				negation(&mut stack);
			},
			'&' => {
				and_operator(&mut stack);
			},
			'|' => {
				or_operator(&mut stack);
			},
			'^' => {
				xor_operator(&mut stack);
			},
			'>' => {
				material_condition_operator(&mut stack);
			},
			'=' => {},
			_ => {
				panic!("invalid input");
			}
		}
	}

	stack.nb_list & 1 == 1 
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn add_number_test() {
		let mut res = Nbstack::new();
		res.add_number(1);
		assert_eq!(res, Nbstack{ nb_list: 1, counter: 1});
		res.add_number(0);
		assert_eq!(res, Nbstack{ nb_list: 2, counter: 2});
	}

	#[test]
	#[should_panic(expected = "val input is not a '0' or a '1', val = 2")]
	fn add_number_invalid_input() {
		let mut test = Nbstack::new();
		test.add_number(2);
	}

	#[test]
	#[should_panic(expected = "buffer limit reached")]
	fn add_number_buffer_limit_test() {
		let mut res = Nbstack::new();
		for _ in 0..=64 {
			res.add_number(0);
		}

	}

	#[test]
	fn extract_number_test() {
		let mut res = Nbstack{ nb_list: 2, counter: 2};
		assert_eq!(res.extract_number(), 0);
		assert_eq!(res.extract_number(), 1);
	}

	#[test]
	#[should_panic(expected = "stack of number is empty")]
	fn extract_number_panic_test() {
		let mut res = Nbstack::new();
		res.extract_number();
	}

	#[test]
	fn true_test() {
		assert_eq!(eval_formula("1"), true);
	}

	#[test]
	fn false_test() {
		assert_eq!(eval_formula("0"), false);
	}

	#[test]
	fn negation_test() {
		assert_eq!(eval_formula("0!"), true);
		assert_eq!(eval_formula("1!"), false);
	}

    #[test]
    fn and_test() {
        assert_eq!(eval_formula("00&"), false);
        assert_eq!(eval_formula("10&"), false);
        assert_eq!(eval_formula("01&"), false);
        assert_eq!(eval_formula("11&"), true);
    }

	#[test]
	fn or_test() {
		assert_eq!(eval_formula("00|"), false);
		assert_eq!(eval_formula("10|"), true);
		assert_eq!(eval_formula("01|"), true);
		assert_eq!(eval_formula("11|"), true);
	}

	#[test]
	fn xor_test() {
		assert_eq!(eval_formula("00^"), false);
		assert_eq!(eval_formula("10^"), true);
		assert_eq!(eval_formula("01^"), true);
		assert_eq!(eval_formula("11^"), false);
	}

	#[test]
	fn material_condition_test() {
		assert_eq!(eval_formula("00>"), true);
		assert_eq!(eval_formula("10>"), true);
		assert_eq!(eval_formula("01>"), false);
		assert_eq!(eval_formula("11>"), true);
	}

	#[test]
	fn logical_equivalence() {
		assert_eq!(eval_formula("00="), true);
		assert_eq!(eval_formula("10="), false);
		assert_eq!(eval_formula("01="), false);
		assert_eq!(eval_formula("11="), true);
	}

	#[test]
	fn or_and_test() {
		assert_eq!(eval_formula("101|&"), true);
		assert_eq!(eval_formula("10|1&"), true);
	}
	#[test]
	fn big_test() {
		assert_eq!(eval_formula("0!00101&|^>="), false);
	}

	#[test]
	fn subject_test() {
		assert_eq!(eval_formula("10&"), false);
		assert_eq!(eval_formula("10|"), true);
		assert_eq!(eval_formula("11>"), true);
		assert_eq!(eval_formula("10="), false);
		assert_eq!(eval_formula("1011||="), true);
	}
}
