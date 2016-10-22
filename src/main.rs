fn add_fifty(n :i32) -> i32 { //can be above or below, works just fine; always have to specify return type as well!
    n + 50; //on last line of function, if you don't put ; the value of the last line will be returned. if you want to return early using "return" keyword. This is rust style, will throw an error
}

fn main() { //can't have 2 mains
    let name = "Banana";
    let mut age: i32 = 5; // assigns type
    age += add_fifty(5); //can't reassign, vars are immutable without mut
    let (number, fruit) = (7, "apples");
    println!("Hi, {}! You are {} years old. You like {} and {}.", name, age, number, fruit);
}
