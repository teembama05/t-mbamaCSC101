fn main() {
    let city_arr:[str;5] = ["abuja", "portharcourt", "maiduguri", "kano", "lagos"];
    println!("array is {:?}", city_arr);
    println!("array size is :{}", city_arr.len());


    for index in 0..8{
        println!("City index {} is located in : {}", index,city_arr[index]);
    }
}
