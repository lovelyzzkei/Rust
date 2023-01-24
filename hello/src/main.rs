// This is the entry of the
// application.
// fn main() {
//     let sum = 2 + 2;
//     let mult = 6 * 6;
//     let mut x = 1;

//     loop {
//         if x == 5 {
//             break;
//         }
//         x = x + 1;
//     }

//     // Display a message to the user
//     println!("{:?}", x);

//     // match
//     // _ : anything else
//     let some_bool = true;
//     match some_bool {
//         true => println!("its true"),
//         _ => println!("its false"),
//     }
// }

// fn add(a: i32, b: i32) -> i32 {
//     return a + b;
// }

fn main() {
    let mut x = 1;

    while x <= 3 {
        println!("{:?}", x);
        x = x + 1;
    }
}
