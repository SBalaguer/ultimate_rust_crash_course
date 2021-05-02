const STARTING_MISSLES: i32 = 8;
const READY_AMOUNT: i32 = 2;

// fn main() {
//     let missiles = STARTING_MISSLES;
//     let ready = READY_AMOUNT;
//     println!("Firing {} of my {} missiles", ready, missiles);
//     let missiles = missiles - ready;
//     println!("{} missiles left", missiles);
// }

//EXTRA CHALLENGE
fn main() {
    let (missiles, ready):(i32, i32) = (STARTING_MISSLES, READY_AMOUNT);
    println!("Firing {} of my {} missiles", ready, missiles);
    println!("{} missiles left", missiles - ready);
}