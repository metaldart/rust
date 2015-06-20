fn main() {
	let s = "hello world";
	let reverse: String = s.chars().rev().collect();
	println!("{}", reverse);

	fn fib(n: i32) -> i32{
		if n == 0 {
			return 0;
		} else if n == 1 {
			return 1;
		} else {
			return fib(n-1) + fib(n-2);
		}}
	println!("{}",fib(6));
}