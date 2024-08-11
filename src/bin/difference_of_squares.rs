

fn square_of_sum(n: u32) -> u32 {
    let sum = n * (n + 1) / 2;
    sum * sum
}

fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

fn main() {
    let n = 10;
    println!("square of sum for the first {} natural numbers: {}", n, square_of_sum(n));
    println!("sum of square for the first {} natural numbers: {}", n, sum_of_squares(n));
    println!("difference of the first {} natural numbers: {}", n, difference(n))
}

// For the sum of natural numbers: Arithmetic Series
// For the sum of the squares of natural numbers: Square Pyramidal Number