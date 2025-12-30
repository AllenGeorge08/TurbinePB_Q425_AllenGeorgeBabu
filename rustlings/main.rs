fn main(){
    panic_if_even(22)
}

fn panic_if_even(i: u64){
    // if i%2 == 0 {
    //     panic!("It's an even number");
    // }
    // println!("It's great");

    for number in 0..i {
        if i%2 == 0 {
            println!("Even")
        }
        println!("Odd")
    }
}