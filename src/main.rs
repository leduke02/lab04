use std::io;

fn main() {
    let weight: f64;
    let total: f64;
    let mut input = String::new();

    println!("Enter the weight:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    weight = input.trim().parse().expect("weight - Not a Number");

    if weight <= 1.0 {
        total = 5.0;
        print!("less than 1: {total}");
    } else if weight <= 5.0 {
        total = 12.0;
        print!("less than 12: {total}");
    } else if weight <= 20.0 {
        total = 25.0;
        print!("less than 25: {total}");
    } else {
        total = 50.0;
        print!("greater than 50: {total}");
    }

let mut destination = String::new();
 println!("Enter the destination: 1. domestic 2. international:");
    io::stdin()
        .read_line(&mut destination)
        .expect("Failed to read line");
let destination: i32 = destination
	.trim()
	.parse()
	.expect("input is not a vaild interger");
	
	
let mut express = String::new();
println!("express shipping? (y/n):"); 
io::stdin()
.read_line(&mut express)
.expect ("Failed to read line");


let mut customer_tier = String::new();
 println!("Enter the customer tier: 1. standard 2. member 3. premium:");
    io::stdin()
        .read_line(&mut customer_tier)
        .expect("failed to read line");
let customer_tier: i32 = customer_tier
        .trim()
        .parse()
        .expect("input is not a vaild interger");

// Rule 3 : destionation + express
	let mut cost = total;
	let is_express = express.trim().eq_ignore_ascii_case("y");

if destination == 1 {
	if is_express {
	cost += 8.0;
	}
}else if destination == 2 {
cost *= 2.0;
 if is_express { 
	cost += 20.0;
}

}
// Rule 4: loyalty discount 
let discount = match customer_tier {
	2 => 0.10, //member
	3 => 0.20, //premium
	_ => 0.0, //standard
};
cost -= cost * discount;

//Rule 5: final cost 
println!("\nFinal cost: ${:.2}", cost);

}




