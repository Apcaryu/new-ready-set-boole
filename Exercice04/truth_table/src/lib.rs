use boolean_evaluation::eval_formula;

#[derive(Debug, Clone)]
struct Variable {
	name: char,
	all_value: Vec<char>
}

impl Variable {
	pub fn new(name: char) -> Variable {
		Variable { name, all_value: Vec::new() }
	}
}

fn search_value(tab: &Vec<char>, target: char) -> bool {
	for iter in tab.iter() {
		if *iter == target {
			return true
		}
	}
	false
}

fn get_variables(formula: &str) -> Vec<char> {
	let mut out = Vec::new();

	for chr in formula.chars() {
		match chr {
			'A'..='Z' => {
				if !search_value(&out, chr) {
					out.push(chr);
				}
			}
			_ => {}
		}
	}
	out
}

fn gen_one_variable(name: char, nb_value: u16, tmp_pow: u32) -> Variable {
	let mut out = Variable::new(name);
	let mut cptr = 0;

	for _ in 0..nb_value {
		if cptr < tmp_pow / 2 {
			out.all_value.push('0');
			cptr += 1;
		} else {
			out.all_value.push('1');
			cptr += 1;
			if cptr == tmp_pow {
				cptr = 0;
			}
		}
	}

	out
}

fn find_variable_in_vector(vec: &Vec<Variable>, target: char) -> Variable {
	for var in vec.iter() {
		if var.name == target {
			return var.clone();
		}
	}

	Variable::new('?')
}

fn calculate_all(formula: &str, mut tab: Vec<Variable>, nb_vars: u8) -> Vec<Variable> {
	let mut res = Variable::new('=');
	println!("{}", nb_vars);
	
	for idx  in 0..2u16.pow(nb_vars as u32) {
		let mut tmp_formula = String::new();
		for chr in formula.chars() {
			if chr.is_ascii_uppercase() {
				tmp_formula.push(find_variable_in_vector(&tab, chr).all_value[idx as usize])
			} else {
				tmp_formula.push(chr);
			}
		}

		if eval_formula(tmp_formula.as_str()) {
			res.all_value.push('1');
		} else {
			res.all_value.push('0');
		}
	}

	tab.push(res);

	tab
}

fn table_value_generation(vars_tab: Vec<char>, formula: &str) -> Vec<Variable> {
	let mut out = Vec::new();
	let mut nb_vars = vars_tab.len();
	let mut cptr = 0;
	// println!("{:?}", vars_tab);
	// println!("{:?}", vars_tab[0]);
	while nb_vars > 0 {
		let tmp_pow = 2u32.pow(nb_vars as u32);
		out.push(gen_one_variable(vars_tab[cptr], 2u16.pow(vars_tab.len() as u32), tmp_pow));

		nb_vars -= 1;
		cptr += 1;
	}

	calculate_all(formula, out, vars_tab.len() as u8)

	// out
}

pub fn print_truth_table(formula: &str) {
	if formula.len() == 0 { panic!("need an input") }
	let vars_tab = get_variables(formula);
	// println!("vars : {:?}", get_variables(formula));
	let tab = table_value_generation(vars_tab, formula);
	println!("{:?}", tab);
    // println!("{}", eval_formula("0!") as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	#[should_panic(expected = "need an input")]
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
