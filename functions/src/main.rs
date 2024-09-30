fn main() {
    let check = check_prime(17);
    println!("{}", check);
    let numbers = [1, 2, 3, 4, 5];
    let max = max_array(&numbers);
    println!("{}", max);
}


// Prime Number Check
// Problem: Write a function that checks if a number is prime.

// Approach:

// A prime number is divisible only by 1 and itself. Check if a number is divisible by any number between 2 and sqrt(n).
// If it's divisible by any number, it's not prime.

fn check_prime(n: u32) -> bool{
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn max_array(arr: &[u32]) -> u32{
    let mut max = arr[0];

    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }
    max
}