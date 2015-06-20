
fn fib(n: usize, a: usize, b: usize) -> usize {
	match n {
			0 => a+b,
			_ => fib(n-1, a+b, a),
		}
}

fn main() {
			for n in (0..10) {
	println!("Fib {} : {}", n, fib(n, 0, 1));
}
}