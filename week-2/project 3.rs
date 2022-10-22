fn main() {
	let P:f64=210_000.00;
	let R:f64=5.00;
	let N:f64=3.00;
	//to calculate the final amount
	let D:f64=(1.0-(R/100.0));
	let D:f64= D*D*D;
	let A:f64=(P*D);
	println!("Value of TV after 3 years {}",A);
}