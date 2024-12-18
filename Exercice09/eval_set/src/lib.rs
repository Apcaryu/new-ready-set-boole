use negation_normal_form::negation_normal_form;

fn get_vars(formula: &str) -> Vec<char> {
	let mut res = Vec::new();

	for chr in formula.chars() {
		match chr {
			'A'..='Z' => {
				let find_var = res.iter().find(|&&x| x == chr);
				match find_var {
					None => {
						res.push(chr);
					},
					_ => {}
				}
			},
			_ => {}
		}
	}

	res
}

fn get_universe(sets: &Vec<Vec<i32>>, vars: Vec<char>) -> Vec<i32> {
	let mut universe = Vec::new();
	for chr in vars {
		let idx: usize = (chr.to_digit(16).unwrap() - 10) as usize;
		let tmp_set = sets[idx].clone();
		for value in tmp_set {
			let in_universe = universe.iter().find(|&&x| x == value);
			match in_universe {
				None => {universe.push(value);},
				_ => {}
			}
		}
	}
	universe
}

fn get_set(stack: &mut Vec<Vec<i32>>, chr: char, sets: &Vec<Vec<i32>>) {
	let idx = chr.to_digit(16).unwrap() - 10; // 10 is the value of A
	stack.push(sets[idx as usize].clone());
}

fn complement(stack: &mut Vec<Vec<i32>>, universe: &Vec<i32>) {
	let var = stack.pop();
	let mut res: Vec<i32> = Vec::new();
	match var {
		Some(var) => {
			for val in universe {
				let finded = var.iter().find(|&&x| x == *val);
				match finded {
					None => {
						res.push(*val);
					},
					_ => {}
				}
			}
		},
		_ => { panic!("set missing") }
	}
	
	stack.push(res);
}

fn intersection(stack: &mut Vec<Vec<i32>>) {
	let var_b = stack.pop();
	let var_a = stack.pop();
	let mut res =Vec::new();

	match var_b {
		Some(value_b) => {
			match var_a {
				Some(_) => {
					for value_a in var_a.unwrap() {
						let finded = value_b.iter().find(|&&x| x == value_a);
						match finded {
							Some(val) => {
								res.push(*val);
							}
							_ => {}
						}
					}
				},
				_ => { panic!("set missing") }
			}
		},
		_ => { panic!("set missing") }
	}

	stack.push(res);
}

fn union(stack: &mut Vec<Vec<i32>>) {
	let var_b = stack.pop();
	let var_a = stack.pop();
	let mut res;

	match var_a {
		Some(values) => {
			res = values;
		},
		_ => { panic!("set missing") }
	}

	match var_b {
		Some(values) => {
			for val in values {
				let finded = res.iter().find(|&&x| x == val);
				match finded {
					None => {
						res.push(val);
					},
					_ => {}
				}
			}
		},
		_ => { panic!("set missing") }
	}

	stack.push(res);
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
	let nnf_formula = negation_normal_form(formula);
	print!("nnf_formula = {} | ", nnf_formula);
	let universe = get_universe(&sets, get_vars(formula));
	let mut stack = Vec::new();

	for chr in nnf_formula.chars() {
		match chr {
			'A'..='Z' => {
				get_set(&mut stack, chr, &sets);
			},
			'!' => {
				complement(&mut stack, &universe);
			},
			'&' => {
				intersection(&mut stack);
			},
			'|' => {
				union(&mut stack);
			},
			_ => {}
		}
	}
	println!("stack {:?}", stack);
    stack.pop().unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

	fn check_sets<T>(left: Vec<T>, right: Vec<T>) 
	where T: std::fmt::Debug + std::cmp::PartialEq + std::marker::Copy
	{
		if left.len() != right.len() {
			panic!("left={:?}\nright={:?}", left, right)
		}

		for val_a in &left {
			let finded = right.iter().find(|&&x| x == *val_a);
			match finded {
				None => { panic!("left={:?}\nright={:?}", left, right) },
				_ => {}
			}
		}
	}

	#[test]
	fn get_vars_tests() {
		check_sets(get_vars("A"), vec!['A']);
		check_sets(get_vars("AB&"), vec!['A', 'B']);
		check_sets(get_vars("AB&B"), vec!['A', 'B']);
		check_sets(get_vars("AFB&&"), vec!['A', 'B', 'F']);
		check_sets(get_vars("AAAAAAAAAAAAAAAAAAAAAAAAAA"), vec!['A']);
	}

	#[test]
	fn universe_tests() {
		let sets = vec![
			vec![0, 1, 2], // A
			vec![0, 3, 4], // B
			vec![3, 8, 2], // C
			vec![4, 0, 3], // D
			vec![5, 10, 4], // E
			vec![7, 5, 6], // F
		];

		check_sets(get_universe(&sets, vec!['A']), vec![0, 1, 2]);
		check_sets(get_universe(&sets, vec!['A', 'B']), vec![0, 1, 2, 3, 4]);
		check_sets(get_universe(&sets, vec!['A', 'B', 'F']), vec![0, 1, 2, 3, 4, 7, 5, 6]);

	}

	#[test]
	fn complement_test() {
		let universe = vec![0, 1, 2];
		let mut stack = vec![vec![0, 1, 2]];
		complement(&mut stack, &universe);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let universe = vec![2, 0, 1];
		stack.push(vec![0, 1, 2]);
		complement(&mut stack, &universe);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let universe = vec![0, 1, 2, 3, 10];
		stack.push(vec![0, 1, 2]);
		complement(&mut stack, &universe);
		assert_eq!(stack.pop().unwrap(), vec![3, 10]);
	}

	#[test]
	fn intersection_test(){
		let mut stack = vec![vec![0, 1, 2], vec![3, 4, 5]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let mut stack = vec![vec![0, 1, 2], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0]);

		let mut stack = vec![vec![2, 1, 0], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0]);

		let mut stack = vec![vec![0, 1, 3], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 3]);

		let mut stack = vec![vec![1, 3, 0], vec![0, 3, 4]];
		intersection(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![3, 0]);
	}

	#[test]
	fn union_test() {
		let mut stack = vec![vec![], vec![]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![]);

		let mut stack = vec![vec![0, 1, 2], vec![]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 1, 2]);

		let mut stack = vec![vec![], vec![0, 1, 2]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0 ,1, 2]);

		let mut stack = vec![vec![0, 1, 2], vec![3, 4, 5]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 1, 2, 3, 4, 5]);

		let mut stack = vec![vec![0, 5, 1], vec![4, 2, 3]];
		union(&mut stack);
		assert_eq!(stack.pop().unwrap(), vec![0, 5, 1, 4, 2, 3]);
	}

    #[test]
    fn subject_tests() {
		let sets = vec![
			vec![0, 1, 2],
		];
		assert_eq!(eval_set("A!", sets), vec![]);
		
        let sets = vec![
			vec![0, 1, 2],
			vec![0, 3, 4],
		];
		assert_eq!(eval_set("AB&", sets), vec![0]);

		let sets = vec![
			vec![0, 1, 2],
			vec![3, 4, 5],
		];
		assert_eq!(eval_set("AB|", sets), vec![0, 1, 2, 3, 4, 5]);

    }

	#[test]
	fn basics_tests() {
		let sets = vec![
			vec![0, 1, 2],
			vec![0, 3, 4],
		];

		assert_eq!(eval_set("A", sets.clone()), vec![0, 1, 2]);
		assert_eq!(eval_set("A!", sets.clone()), vec![]);
		assert_eq!(eval_set("AB&", sets.clone()), vec![0]);
		assert_eq!(eval_set("AB|", sets.clone()), vec![0, 1, 2, 3, 4]);
		assert_eq!(eval_set("AB^", sets.clone()), vec![1, 2, 3, 4]);
		assert_eq!(eval_set("AB>", sets.clone()), vec![3, 4, 0]);
		assert_eq!(eval_set("AB=", sets.clone()), vec![]);
	}


	#[test]
	fn complex_tests() {
		let sets = vec![
			vec![0, 1, 2], // A
			vec![0, 3, 4], // B
			vec![3, 8, 2], // C
			vec![4, 0, 3], // D
			vec![5, 10, 4], // E
			vec![7, 5, 6], // F
			// Universe = [0, 1, 2, 3, 4, 8, 5, 10, 7, 6]
		];

		check_sets(eval_set("AD|B|", sets.clone()), vec![0, 1, 2, 4, 3]);
		check_sets(eval_set("D!B&A^", sets.clone()), vec![0, 1, 2]);
		check_sets(eval_set("ABC&&", sets.clone()), vec![]);
		check_sets(eval_set("FED||", sets.clone()), vec![7, 5, 6, 10, 4, 0, 3]);
		check_sets(eval_set("ACE^^", sets.clone()), vec![3, 8, 5, 10, 4, 0, 1]);
		check_sets(eval_set("FDB>>", sets.clone()), vec![7, 5, 6, 0, 3, 4]);
		check_sets(eval_set("ABF==", sets.clone()), vec![]);
	}
}
