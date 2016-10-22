fn main() {
   let name = "Banana";
   let mut age = 5;
   age += 5; //can't reassign, vars are immutable without mut
   let (number, fruit) = (7, "apples");
   println!("Hi, {}! You are {} years old. You like {} and {}.", name, age, number, fruit);
}
