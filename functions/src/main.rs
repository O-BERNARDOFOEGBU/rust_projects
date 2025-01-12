fn main() {
    println!("Hello World");
    another_function();
    parameter(10);
    print_labeled_measurements(5, 'h')
}

fn another_function() {
    println!("Another function");
}

fn parameter(x:i32) {
    println!("The value of x is: {x}")
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement value is {value} and the unit label is {unit_label} ")
}