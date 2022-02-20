

fn main() {
    // const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    // let mut x = 5;
    // print!("The value of x is : {}", x);
    // x = 6;
    // print!("The value of x is: {}", x);

    let x = 5;
    let x = x+1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

}
