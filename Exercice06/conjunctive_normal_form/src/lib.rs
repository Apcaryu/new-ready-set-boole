use negation_normal_form::negation_normal_form;

fn conjunctive_normal_form(formula: &str) -> String {
	let nnf_formula = negation_normal_form(formula);
	nnf_formula
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
		assert_eq!(conjunctive_normal_form("A"), "A");
		assert_eq!(conjunctive_normal_form("A!"), "A!");
		assert_eq!(conjunctive_normal_form("AB&"), "AB&");
		assert_eq!(conjunctive_normal_form("AB|"), "AB|");
		assert_eq!(conjunctive_normal_form("AB^"), "A!B!|AB|&");
		assert_eq!(conjunctive_normal_form("AB>"), "A!B|");
		assert_eq!(conjunctive_normal_form("AB="), "A!B|AB!|&");
	}

	#[test]
	fn to_cnf_only_use_nnf_function() {
		assert_eq!(conjunctive_normal_form("A!!"), "A");
		assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
		assert_eq!(conjunctive_normal_form("AB|"), "A!B!&");
		assert_eq!(conjunctive_normal_form("A!B>"), "AB|");

	}
}
