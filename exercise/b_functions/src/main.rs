// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;
    // 1. Try running this code with `cargo run` and take a look at the error.
    //
    // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
    // doing `cargo run` should succeed and print something out.
    let area = area_of(width, height);

    println!("Area is {}", area);

    // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
    //    Create the `volume` function!  It should:
    //    - Take three arguments of type i32
    //    - Multiply the three arguments together
    //    - Return the result (which should be 280 when you run the program).
    //
    // If you get stuck, remember that this is *very* similar to what `area_of` does.
    //
    println!("Volume is {}", volume(width, height, depth));
}

fn area_of(x: i32, y: i32) -> i32 {
    // 2a. Fix this function to correctly compute the area of a rectangle given
    // dimensions x and y by multiplying x and y and returning the result.
    //
    // return x*y;
    // Challenge: The previous line is not idiomatic (not recommended best practice).
    //            Run `cargo clippy`, figure out what's wrong, and fix it.  Once it is fixed,
    //            `cargo clippy` won't return areas, and `cargo run` will still produce the same
    //            output. See also https://github.com/rust-lang/rust-clippy
    x*y
}

fn volume (x:i32, y:i32, z:i32) -> i32{
    x*y*z
}