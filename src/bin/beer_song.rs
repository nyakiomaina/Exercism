fn print_verse(n: i32) {
    if n > 1 {
        println!("{} bottles of beer on the wall, {} bottles of beer.", n, n);
        println!("Take one down and pass it around, {} bottles of beer on the wall.\n", n - 1);
    } else if n == 1 {
        println!("1 bottle of beer on the wall, 1 bottle of beer.");
        println!("Take one down and pass it around, no more bottles of beer on the wall.\n");
    } else {
        println!("No more bottles of beer on the wall, no more bottles of beer.");
        println!("Go to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
}

fn main() {
    for i in (0..100).rev() { // start..end (includes the start value, exclude the end value)
        print_verse(i);
    }
}
