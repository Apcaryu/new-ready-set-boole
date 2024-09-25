pub fn eval_formula(formula: &str) -> bool {
	false
}

#[cfg(test)]
mod tests {
    use super::*;

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
		assert_eq!(eval_formula("!0"), true);
		assert_eq!(eval_formula("!1"), false);
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
		assert_eq!(eval_formula("00ˆ"), false);
		assert_eq!(eval_formula("10ˆ"), true);
		assert_eq!(eval_formula("01ˆ"), true);
		assert_eq!(eval_formula("11ˆ"), false);
	}

	#[test]
	fn material_condition_test() {
		assert_eq!(eval_formula("00>"), true);
		assert_eq!(eval_formula("10>"), false);
		assert_eq!(eval_formula("01>"), true);
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
		assert_eq!(eval_formula("0!00101&|ˆ>="), false);
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
