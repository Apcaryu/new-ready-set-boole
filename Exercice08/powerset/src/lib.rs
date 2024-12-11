// this function assumed that the set passed as parameter is valid (no duplicate elements)
// set size max = 63 elements 
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
	let mut out: Vec<Vec<i32>> = Vec::new();
	let set_len = set.len();
	if set_len >= 64 {
		panic!("Too many element in set, size max of set is 63, your set size = {}", set_len)
	}

	out.push(vec![]);

	for nb_res in 1..((1 as u64) << set_len) {
		let mut subset = Vec::new();
		for elem_pos in 0..set_len {
			if nb_res & (1 << elem_pos) != 0 {
				subset.push(set[elem_pos]);
			}
		}
		out.push(subset);
	}

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_test() {
        assert_eq!(powerset(vec![]), vec![vec![]]);
		assert_eq!(powerset(vec![1]), vec![vec![], vec![1]]);
		assert_eq!(powerset(vec![1, 2]), vec![vec![], vec![1], vec![2], vec![1, 2]]);
		assert_eq!(powerset(vec![1, 2, 3]), vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]]);
		assert_eq!(powerset(vec![10, 42, -84572965]), vec![vec![], vec![10], vec![42], vec![10, 42], vec![-84572965], vec![10, -84572965], vec![42, -84572965], vec![10, 42, -84572965]]);
    }

	#[test]
	fn size_test() {
		assert_eq!(powerset(vec![1, 2, 3, 4]).len(), 16);
		assert_eq!(powerset(vec![1, 2, 3, 4, 5]).len(), 32);
		assert_eq!(powerset(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).len(), 1024);
	}

	#[test]
	#[should_panic]
	fn too_large_set() {
		powerset(vec![1,2,3,4,5,6,7,8,9,10,
			11,12,13,14,15,16,17,18,19,20,
			21,22,23,24,25,26,27,28,29,30,
			31,32,33,34,35,36,37,38,39,40,
			41,42,43,44,45,46,47,48,49,50,
			51,52,53,54,55,56,57,58,59,60,
			61,62,63,64]);
	}
}
