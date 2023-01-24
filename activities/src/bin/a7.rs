// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


// Demo
enum Color {
    Red,
    Blue,
    Green
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Green"),
        Color::Green => println!("Blue"),
    }
}

fn main() {
    let red = Color::Red;
    print_color(red);

}
