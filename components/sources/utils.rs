pub fn ne_assign<T> (a: &mut T, b: T) -> bool where T: PartialEq {
	if &b == a {
		false
	} else {
		*a = b;
		true
	}
}
