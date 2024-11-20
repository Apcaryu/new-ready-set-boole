fn conjunctive_normal_form(formula: &str) -> String {
	String::new()
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
}
