use negation_normal_form::{negation_normal_form, separator};

fn conjunction(stack: &mut Vec<String>) {
	let mut var_b = conjunctive_normal_form(stack.pop().unwrap().as_str());
	let mut var_a = conjunctive_normal_form(stack.pop().unwrap().as_str());
	while var_a.chars().last() == Some('&') {
		var_a.pop();
		var_b = format!("{}&", var_b);
	}

	stack.push(String::from(format!("{}{}&", var_a, var_b)));
}

fn de_morgan_law(φ: String, ψ: String) -> (String, String) {
	let (psi, psi_p) = separator(ψ);
	let var_a = conjunctive_normal_form(format!("{}{}|", φ, psi).as_str());
	let var_b = conjunctive_normal_form(format!("{}{}|", φ, psi_p).as_str());
	(var_a, var_b)
}

fn disjunction(stack: &mut Vec<String>) {
	let mut var_b = conjunctive_normal_form(stack.pop().unwrap().as_str());
	let mut var_a = conjunctive_normal_form(stack.pop().unwrap().as_str());
	
	// while var_a.chars().last() == Some('|') {
	// 	var_a.pop();
	// 	var_b = format!("{}|", var_b);
	// }

	if var_a.chars().last() == Some('&') {
		(var_a, var_b) = de_morgan_law(var_b, var_a);
		// println!("{}{}&", var_a, var_b);
		stack.push(format!("{}{}&", var_a, var_b));
		return ();
	} else if var_b.chars().last() == Some('&') {
		(var_a, var_b) = de_morgan_law(var_a, var_b);
		// println!("{}{}&", var_a, var_b);
		stack.push(format!("{}{}&", var_a, var_b));
		return ();
	}

	while var_a.chars().last() == Some('|') {
		var_a.pop();
		var_b = format!("{}|", var_b);
	}

	stack.push(String::from(format!("{}{}|", var_a, var_b)));
}

pub fn conjunctive_normal_form(formula: &str) -> String {
	let nnf_formula = negation_normal_form(formula);
	let mut stack = Vec::new();

	for chr in nnf_formula.chars() {
		match chr {
			'A'..='Z' => {
				stack.push(String::from(chr));
			},
			'!' => {
				let tmp = stack.pop().unwrap();
				stack.push(String::from(format!("{}!", tmp)));
			},
			'&' => {
				conjunction(&mut stack);
			},
			'|' => {
				disjunction(&mut stack);
			},
			_ => {
				panic!("invalid input");
			}
		}
	}

	/* 
	// idea for a relecture but actualy overflow
	// let tmp = String::from(conjunctive_normal_form(stack.pop().unwrap().as_str()));
	// stack.push(tmp);
	*/
	match stack.pop() {
		Some(val) => return val,
		None => {
			panic!("Stack is empty")
		}
	}
	// nnf_formula
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
		assert_eq!(conjunctive_normal_form("AB>"), "A!B|");
		assert_eq!(conjunctive_normal_form("AB^"), "AA!|AB|&B!A!|B!B|&&"); // TODO need to return in cnf function for this result: "AA!|AB|B!A!|B!B|&&&"
		assert_eq!(conjunctive_normal_form("AB="), "AA!|AB!|&BA!|BB!|&&"); // TODO need to return in cnf function for this result: "AA!|AB!|BA!|BB!|&&&"
	}

	#[test]
	fn mid_tests() {
		assert_eq!(conjunctive_normal_form("AB|AB&|"), "ABA||ABB||&");
		assert_eq!(conjunctive_normal_form("AB|AB&&!"), "A!B!A!||A!B!B!||&");
	}

	#[test]
	fn explosive_test() {
		assert_eq!(conjunctive_normal_form("AB^AB^&"), "AA!|AB|B!A!|B!B|AA!|AB|B!A!|B!B|&&&&&&&");
	}

	#[test]
	fn to_cnf_only_use_nnf_function() {
		assert_eq!(conjunctive_normal_form("A!!"), "A");
		assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
		assert_eq!(conjunctive_normal_form("A!B>"), "AB|");
		assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
	}

	#[test]
	fn subject_tests() {
		assert_eq!(conjunctive_normal_form("AB&!"), "A!B!|");
		assert_eq!(conjunctive_normal_form("AB|!"), "A!B!&");
		assert_eq!(conjunctive_normal_form("AB|C&"), "AB|C&");
		assert_eq!(conjunctive_normal_form("AB|C|D|"), "ABCD|||");
		assert_eq!(conjunctive_normal_form("AB&C&D&"), "ABCD&&&");
		assert_eq!(conjunctive_normal_form("AB&!C!|"), "A!B!C!||");
		assert_eq!(conjunctive_normal_form("AB|!C!&"), "A!B!C!&&");
	}

}
