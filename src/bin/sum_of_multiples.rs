use std::collections::HashSet;

/// Calculates the sum of all unique multiples of given base values that are less than the specified level.
fn calculate_energy_points(level: u32, base_values: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    // Iterate over each base value and calculate its multiples
    for &base in base_values {
        let mut multiple = base;
        while multiple < level {
            multiples.insert(multiple);
            multiple += base;
        }
    }

    // Sum all the unique multiples
    multiples.iter().sum()
}

fn main() {
    let level = 67;
    let base_values = vec![5, 6, 7, 8, 9, 2, 3];  // Base values of the magical items found
    let energy_points = calculate_energy_points(level, &base_values);
    println!("Energy points awarded: {}", energy_points);  // Output: Energy points awarded: 78
}

// use std::io;
// use std::collections::HashSet;
//
// fn calculate_energy_points(level: u32, base_values: Vec<u32>) -> u32 {
//     let mut multiples = HashSet::new();
//
//     for base in base_values {
//         let mut multiple = base;
//         while multiple < level {
//             multiples.insert(multiple);
//             multiple += base;
//         }
//     }
//
//     multiples.iter().sum()
// }
//
// fn main() {
//     let mut input = String::new();
//
//     println!("Enter the level number:");
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     let level: u32 = input.trim().parse().expect("Please type a number!");
//
//     input.clear(); // Clear the input buffer for next input
//
//     println!("Enter base values of magical items found (separated by space or comma):");
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//
//     // Handling both spaces and commas as delimiters
//     let base_values: Vec<u32> = input.trim().split(|c: char| c == ',' || c.is_whitespace())
//         .filter(|s| !s.is_empty())
//         .map(|s| s.parse().expect("Please type a number!"))
//         .collect();
//
//     let energy_points = calculate_energy_points(level, base_values);
//     println!("Energy points awarded: {}", energy_points);
// }
