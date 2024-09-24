pub fn adder(a: u32, b: u32) -> u32 {
    let mut mem = a & b;
	let mut res = a ^ b;

	while mem != 0 {
		let shift_mem = mem << 1;
		if shift_mem == 0 { panic!("adder overflow") }
		mem = shift_mem & res;
		res = res ^ shift_mem;
	}
	res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_number() {
        let result = adder(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
	fn two_numbers() {
		let res = adder(1, 5);
		assert_eq!(res, 1 + 5);
	}

	#[test]
	fn multi_adder() {
		let init = 4;
		let nb_add = 8;
		let mut res_adder = init;
		let mut res_plus = init;

		for _ in 1..=10 {
			res_adder = adder(res_adder, nb_add);
			res_plus = res_plus + nb_add;
		}
		assert_eq!(res_adder, res_plus);
	}

	#[test]
	#[should_panic(expected = "adder overflow")]
	fn overflow() {
		adder(std::u32::MAX, 1);
	}

}
