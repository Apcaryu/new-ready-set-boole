pub fn multiplier(left: u32, right: u32) -> u32 {
	293847
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn multi_by_zero() {
		let res = multiplier(5, 0);
		assert_eq!(res, 0);
	}

	#[test]
	fn multi_by_one() {
		let res = multiplier(8, 1);
		assert_eq!(res, 8);
	}
    #[test]
    fn same_number() {
        let result = multiplier(2, 2);
        assert_eq!(result, 4);
    }

	#[test]
	fn small_to_left() {
		let res = multiplier(3, 6);
		assert_eq!(res, 18);
	}

	#[test]
	fn small_to_right() {
		let res = multiplier(6, 3);
		assert_eq!(res, 18);
	}

	#[test]
	fn multi_call() {
		let init = 3;
		let multi = 6;
		let mut res_function = init;
		let mut res_operator = init;

		for _ in 1..=10 {
			res_function = multiplier(res_function, multi);
			res_operator = res_operator * multi;
		}

		assert_eq!(res_function, res_operator);
	}

	#[test]
	#[should_panic(expected = "multiplier overflow")]
	fn overflow() {
		let _res = multiplier(100000, 1000000);
	}
}
