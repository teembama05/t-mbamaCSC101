fn main() {
	//to find the actual amount for each item
	let T=2*450_000;
	let M=1*1_500_000;
	let H=3*750_000;
	let D=3*2_850_000;
	let A=1*250_000;
	//to find the sum of the sales record
	let sum=(T+M+H+D+A);
	println!("The Sum of the sales record is {}",sum);
	//to find the average of the sales record
	let avg=(sum/10);
	println!("The Average of the sales record is {}",avg);

}