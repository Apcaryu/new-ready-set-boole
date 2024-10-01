pub fn print_truth_table(formula: &str) {
    return;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	#[should_panic(expected = "Need an input")]
    fn no_input() {
		print_truth_table("");
	}

	#[test]
	#[should_panic(expected = "invalid input")]
	fn invalid_input() {
		print_truth_table("AN@");
	}

	#[test]
	#[should_panic(expected = "missing operator")]
	fn missing_operator() {
		print_truth_table("AB|C");
	}

	#[test]
	#[should_panic(expected = "missing value")]
	fn missing_variable() {
		print_truth_table("A&");
	}
}
