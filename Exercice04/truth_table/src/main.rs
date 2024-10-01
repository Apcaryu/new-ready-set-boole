use truth_table::print_truth_table;

fn print_result(formula: &str) {
	println!("{}", formula);
	print_truth_table(formula);
}

fn main() {

	// one_varible()
	println!("---------- ONE VARIABLE ----------\n");
	print_result("A!");
	print_result("AA|");
	println!("\n--------------------\n");

	// two_variable_base()
	println!("---------- TWO VARIABLE BASE ----------\n");
	print_result("AB&");
	print_result("AB|");
	print_result("AB^");
	print_result("AB>");
	print_result("AB=");
	println!("\n--------------------\n");

	// two_variable_tricky()
	println!("---------- TWO VARIABLE TRICKY ----------\n");
	print_result("AB&A|");
	print_result("AB^B>");
	print_result("AB=A^");
	println!("\n--------------------\n");

	// variable_base()
	println!("---------- MULTI VARIABLE ----------\n");
	print_result("ANY|&!");
	print_result("AAAAAH|!&!&!&!&!");
	println!("\n--------------------\n");

	// subject_test()
	println!("---------- SUBJECT TEST ----------\n");
	print_result("AB&C|");
	println!("\n--------------------\n");

}