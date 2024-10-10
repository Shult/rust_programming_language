use std::io;

fn main() {
    println!("Fibonacci suite with Rust!");
    let mut fibonacci_sequence: Vec<i64> = Vec::new();

    loop {
        println!("Please enter the number corresponding to the position in the Fibonacci sequence you would like to know!");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: usize = n.trim().parse().expect("Number");
        let mut i: usize = 2;
        
        fibonacci_sequence.clear();
        fibonacci_sequence.push(0);
        fibonacci_sequence.push(1);

        while i < n {
            fibonacci_sequence.push(fibonacci(fibonacci_sequence[i-1], fibonacci_sequence[i-2]));
            i += 1;
        }
        println!("The {}th number of the fibonacci suite is {}!",n, fibonacci_sequence[n-1] )
    }
}


fn fibonacci(n1: i64, n2: i64) -> i64 {
    n1+n2
}