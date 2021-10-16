fn main() {
    let _a: i32=5;

    let _b: [i32; 3] = [1, 2, 3];

    let _c: [&str; 2] = ["engineer", "akn"];

    let _num1: i32 = 3278;

    let _num1: f32 = 32.78; 

    let _num1 = 4;

    let _a: Vec<i32> = Vec::new();

    let mut _num2 = 56;

    _num2 =57;

    let name1: &'static str = "engineer akn";
    //let name1: &str = "engineer akn";
    println!("{}", name1);

    let mut name2: String = String::new();

    name2 = name2 + "engineer";
    name2.push_str(" akn");
    println!("{}", name2);


    let name3: &str = &name1[..8];
    let name4: &str = &name1[9..];

    println!("{} {}", name3, name4);



    let name = String::from("engineer man");

    let size = get_length(&name);
    println!("{} is {} characters", name, size);
    
    // this dont work because program deletes variable after use 
    // because of that you need to pass the reference of the variable 
    // if you dont want it deleted
    //take_ownership(name);
    borrow_name(&name);
    println!("{}", name); 

    println!("{}", add(1,2));
    println!("{}", add(26,24));

    let mut num = 1;

    inc(&mut num);
}

fn get_length(s: &str) -> usize {
    s.chars().count()
    //return s.chars().count();

}
// fn take_ownership(s: String) {
//     print!("{}", s);
// }
fn borrow_name(s: &str) {
    print!("{}", s);
}
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn inc(n1: &mut i32){
    *n1 = *n1 + 1
}












