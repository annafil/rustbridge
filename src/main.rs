fn add_fifty(n :i32) -> i32 { 
	//can be above or below, works just fine; always have to specify return type as well!
    n + 50
    //on last line of function, if you don't put ; the value of the last line will be returned. if you want to return early using "return" keyword. This is rust style, will throw an error
}

fn main() { //can't have 2 mains
    let name = "Banana";
    let mut age: i32 = 5; // assigns type
    age += add_fifty(5); //can't reassign, vars are immutable without mut
    let (number, fruit) = (7, "apples");
    println!("Hi, {}! You are {} years old. You like {} and {}.", name, age, number, fruit);

    let movie_age = 17u32; // unsigned, positive 32bit, rust style to underscore multiple words, snake case warning

    if movie_age < 13 {
    	println!("You may see G or PG Movies");
    } else if movie_age < 17 {
    	println!("You may see G, PG, or PG-13 movies");
    } else {
    	println!("You are old.");
    	println!("You may see G, PG, PG-13, or R movies");
    }

    match_function(2);
}

// each part of if condition, if it was in function, would have to return the same type in order to not get errors

// kebab case => movie-age 

fn match_function(n :u32) { // no return because not returning things, just printing 
	match n {
		0...12 => println!("You may see G or PG Movies"),
		13...16 => println!("You may see G, PG, or PG-13 Movies"),
		_ => {
			println!("You are old");
			println!("You may see G, PG, PG-13 or R movies");
		},
	}
}