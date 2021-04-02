use inline_python::python;

fn main() {
	let mut x = 42;
	python! {
		#x = #x - 6
		assert 'x == 42
		assert #x == 36

	}
    assert_eq!(x, 36);
}
