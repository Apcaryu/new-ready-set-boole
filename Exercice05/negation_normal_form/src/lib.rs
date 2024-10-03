pub fn negation_normal_form(formula: &str) -> String {
	String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn base_case() {
		assert_eq!(negation_normal_form("A!"), "A!");
		assert_eq!(negation_normal_form("AB&"), "AB&");
		assert_eq!(negation_normal_form("AB|"), "AB|");
		assert_eq!(negation_normal_form("AB^"), "AB!&A!B&|");
		assert_eq!(negation_normal_form("AB>"), "A!B|");
		assert_eq!(negation_normal_form("AB="), "AB&A!B!&|");
	}

	#[test]
	fn mid_case() {
		assert_eq!(negation_normal_form("A!!"), "A");
		assert_eq!(negation_normal_form("AB&!"), "A!B!|");
		assert_eq!(negation_normal_form("AB|!"), "A!B!&");
	}

	#[test]
	fn tricky_case() {
		assert_eq!(negation_normal_form("A!B>A&"), "AB|A&");
		assert_eq!(negation_normal_form("AB&!C|!"),"AB&C!&");
	}
	
	#[test]
	fn already_in_nnf() {
		assert_eq!(negation_normal_form("AB!&A!B&|"), "AB!&A!B&|");
		assert_eq!(negation_normal_form("A!B|"), "A!B|");
		assert_eq!(negation_normal_form("AB&A!B!&|"), "AB&A!B!&|");
	}

	#[test]
	#[should_panic(expected = "invalid input")]
	fn invalid_input() {
		negation_normal_form("AB@");
	}

	#[test]
	#[should_panic(expected = "missing operator")]
	fn missing_operator() {
		negation_normal_form("AB!");
	}

	#[test]
	#[should_panic(expected = "missing variable")]
	fn missing_variable() {
		negation_normal_form("A&");
	}
}
