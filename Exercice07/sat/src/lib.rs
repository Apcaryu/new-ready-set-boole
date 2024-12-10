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

	fn _get_value(&self, idx: usize) -> char {
		self.all_value[idx]
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

fn gen_one_variable(name: char, nb_value: u16, tmp_pow: u64) -> Variable {
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

fn calculate_all(formula: &str, tab: Vec<Variable>, nb_vars: u8) -> bool {
	
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
			return true
		}
	}

	false
}

fn gen_var_tab(vars: Vec<char>) -> Vec<Variable> {
	let mut out = Vec::new();
	let mut nb_vars = vars.len();
	let mut cptr = 0;
	
	while nb_vars > 0 {
		let tmp_pow = 2u64.pow(nb_vars as u32);
		out.push(gen_one_variable(vars[cptr], 2u16.pow(vars.len() as u32), tmp_pow));

		nb_vars -= 1;
		cptr += 1;
	}

	out
}

pub fn sat(formula: &str) -> bool {
	if formula.len() == 0 { panic!("need an input") }
	let vars_tab = gen_var_tab(get_variables(formula));
	// println!("{:?}", vars_tab);
	let vars_len = vars_tab.len();
	
	calculate_all(formula, vars_tab, vars_len as u8)
	// true
}

#[cfg(test)]
mod tests {
    use super::*;
	
	#[test]
	fn one_var() {
		assert!(sat("AA&"));
		assert!(sat("AA|"));
		assert!(!sat("AA^"));
		assert!(sat("AA>"));
		assert!(sat("AA="));
		assert!(!sat("AA!&"));
		assert!(sat("AA!|"));
		assert!(sat("AA!^"));
		assert!(sat("AA!>"));
		assert!(!sat("AA!="));
	}

	#[test]
    fn basic_satisfiable() {
		assert!(sat("A"));
		assert!(sat("A!"));
		assert!(sat("AB|"));
		assert!(sat("AB&"));
		assert!(sat("AB^"));
		assert!(sat("AB>"));
		assert!(sat("AB="));
	}

	#[test]
	fn mid_test() {
		assert!(sat("AA!&B|"));
		assert!(!sat("AA!&BB!&|"));
		assert!(sat("AA^B|"));
		assert!(!sat("AA^B&"));
		assert!(sat("AB=C&"));
		assert!(!sat("AB=AB=^"));
		assert!(sat("ABC||"));
		assert!(!sat("AB&CC^&"));
	}

	#[test]
	fn explosive_test() {
		// assert!(sat("ABCDEFGHIJKLMNO&&&&&&&&&&&&&&")); // Actual max
		// assert!(sat("ABCDEFGHIJKLMNOP&&&&&&&&&&&&&&&"));
		// assert!(sat("ABCDEFGHIJKLMNOPQRSTUVWXYZ&&&&&&&&&&&&&&&&&&&&&&&&&"));
		// assert!(!sat("AA!&BB!&CC!&DD!&EE!&FF!&GG!&HH!&II!&JJ!&KK!&LL!&MM!&NN!&OO!&PP!&QQ!&RR!&SS!&TT!&UU!&VV!&WW!&XX!&YY!&ZZ!&&&&&&&&&&&&&&&&&&&&&&&&&&"))
	}

}
