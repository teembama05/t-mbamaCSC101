fn main() {
	let P:f64=520_000_000.00;
	let R:f64=10.00;
	let N:f64=5.00;
	//calculation for the amount
	let A:f64=P*(1 + (R/100))*N;
	println!("Amount is {}",A);
	//calculation for compund interest
	let C-I:f64=A-P;
	println!("Compound Interest is {}",C-I);

}