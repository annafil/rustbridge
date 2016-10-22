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

    // panic!("Aaaaaa!"); this will end program and panic 

    match_function(2);

    let color = [255, 0, 255]; // fixed count of items, only have 3 items always
    println!("The color is {:?}", color[0]); // :? print variable with debugging format, previously display format, there is no display for arrays :#? pretty debug  

    //most of the time you want a vector rather than array 
    let mut prices = vec![30,100,2]; // mutating = borrowing, can't add new value without mut, must be same type, infer type from the first one
    prices[0] = 25;
    prices.push(40);
    println!("All prices are: {:#?}", prices); // pretty debug will put on each line

    loops_iterators(age);

    enums();

}

// each part of if condition, if it was in function, would have to return the same type in order to not get errors

// kebab case => movie-age 

fn loops_iterators(age :i32) {

	for i in 0..10 { // exclusive range 
		println!("NUmber {}", i);
	}

	let names = vec!["Carol", "Jake", "Marylou", "Bruce"];

	let name  = "Question"; // this is not the same name as inside the loop 

	for name in names.iter() { // no variable name after the loop
		println!("Hi {}!", name); //can access vars outside the for loop
		let age = add_fifty(age);
		println!("{}", age);
	}

	println!("name {}", name); // prints original name 

	//loop is an infinite loop, while, break exists the current loop

	for i in (0..10).filter(|x| x % 2 == 0) { // keeps what matches
		println!("filter = {}", i);
	}


	for i in (0..10).map(|x| x * x) { 
		println!("map = {}", i);
	}

	let sum = (0..10).fold(0, |acc,x| acc + x); // reduce, accumulator, starting at base value pass accumulator value iterating over, value inside block will be returned
	println!("sum = {}", sum);
	println!("sum = {}", (0..10).sum::<i32>()); //how does this typing work? 

}

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

fn add_fifty(n :i32) -> i32 { 
	//can be above or below, works just fine; always have to specify return type as well!
    n + 50
    //on last line of function, if you don't put ; the value of the last line will be returned. if you want to return early using "return" keyword. This is rust style, will throw an error
}

fn enums() {
	enum TrafficLight {
		Red,
		Yellow,
		Green,
	}

	let light = TrafficLight::Yellow;

	match light {
		TrafficLight::Red => println!("STOP!"),
		//TrafficLight::Yellow => println!("Slow down!"),
		TrafficLight::Green | TrafficLight :: Yellow => println!("Go go go!"),
	}

	//enums can hold values! different variants different things 

	enum GameType {
	    SinglePlayer,
	    MultiPlayer(u32),
	}

	let game = GameType::MultiPlayer(10);
	match game { // curly braces start a new scope 
	    GameType::SinglePlayer => println!("How about solitaire?"),
	    GameType::MultiPlayer(2) => println!("How about checkers?"),
	    GameType::MultiPlayer(4) => println!("How about bridge?"),
	    GameType::MultiPlayer(num) => {
	        println!("How about {}-player tag?", num)
	    },
	}

	let mut instructors = vec!["Carol"];

	let a = instructors.pop();
	println!("a is {:?}", a );

	let b = instructors.pop();
	println!("b is {:?}", b);

	let a = Some("Carol");

	let name = a.expect("No name present");
	println!("Name is {} bytes long", name.len());

	let b: Option<&str> = None;

	match b {
		Some(name) => {
			println!("Other name is {} bytes long", name.len())
		},
		None => {
			println!("No name!")
		}
	}
}

