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
    	println!("You are {} years old. You may see G or PG Movies", movie_age);
    } else if movie_age < 17 {
    	println!("You are {} years old. You may see G, PG, or PG-13 movies", movie_age);
    } else {
    	println!("You are {} years old.", movie_age);
    	println!("You may see G, PG, PG-13, or R movies");
    }

    match_function(2);

    let mut color = [255, 0, 255]; // fixed count of items, only have 3 items always
    color[0] = 100;
    println!("The color is {:?}", color);
}

// each part of if condition, if it was in function, would have to return the same type in order to not get errors

// kebab case => movie-age 

fn match_function(n :u32) { // no return because not returning things, just printing 
	match n {
		0...12 => println!("You are {} years old. You may see G or PG Movies", n),
		13...16 => println!("You are {} years old. You may see G, PG, or PG-13 Movies", n),
		_ => { // if you don't have this, will throw an error, you have to explicitly specify all cases 
			println!("You are {} years old. You are old", n);
			println!("You are {} years old. You may see G, PG, PG-13 or R movies", n);
		},
	}
}

