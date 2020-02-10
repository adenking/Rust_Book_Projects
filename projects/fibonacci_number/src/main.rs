use std::io;

fn main() {
    loop {
        println!("What (n)th fibonacci would you like?");

        let mut input_fibo = String::new();
        io::stdin()
            .read_line(&mut input_fibo)
            .expect("Failed to read line.");
        
        let input_fibo: u32 = match input_fibo.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a Valid Number");
                continue;
            }
        };

    
        let fibo_result = calculate_fibo(input_fibo);
        println!("The {}th Fibonacci is {}", input_fibo, fibo_result);
        
    }
}

fn calculate_fibo(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => calculate_fibo(n - 1) + calculate_fibo(n - 2),
    }

}