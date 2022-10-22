fn main() {
	let P:f64= 520_000_000.00;
	let R:f64= 10.00;
	let N:f64= 5.00;
	//to calculate the final amount
	let D:f64=(1.0+(R/100.0));
	let D:f64= D*D*D;
	let A:f64=(P*D);
	println!("The Amount after 5 years {}",A);
	let C.I:f64= A-P
	println!("The Compund Interese is {}",C.I );
}