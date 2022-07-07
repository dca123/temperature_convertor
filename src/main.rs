use std::io;
fn main() {
    const C_TO_F_LABELS: (&str, &str) = ("C", "F");
    const F_TO_C_LABELS: (&str, &str) = ("F", "C");

    loop {
        println!("Enter your choice:");
        println!("1. F -> C");
        println!("2. C -> F");
        println!("3. Exit");

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => {
                if num == 3 {
                    break;
                } else if num != 1 && num != 2 {
                    println!("Only 1 and 2 are accepted");
                    continue;
                }
                num
            }
            Err(_) => continue,
        };

        let input_label = if selection == 1 {
            F_TO_C_LABELS.0
        } else {
            C_TO_F_LABELS.0
        };

        let output_label = if selection == 1 {
            F_TO_C_LABELS.1
        } else {
            C_TO_F_LABELS.1
        };

        println!("Enter your temperature in {input_label}");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().expect("Please type a number");

        let converted: f64 = if selection == 1 {
            f_to_c(input)
        } else {
            c_to_f(input)
        };
        println!("{input}{input_label} is {converted}{output_label}");
    }
}

fn f_to_c(input: i32) -> f64 {
    (input - 32) as f64 * 0.5556
}

fn c_to_f(input: i32) -> f64 {
    input as f64 * 1.8 + 32 as f64
}
