mod tests {
	use head_of_departament::add;

	#[test]
	fn simple_add() {
		assert_eq!(add(5, 4), 9);
	}

}
