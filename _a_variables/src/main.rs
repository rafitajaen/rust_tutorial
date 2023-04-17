const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // let missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY_AMOUNT;
    let (missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {ready} of my {missiles} missiles..");

    // let missiles: i32 = missiles - ready;
    println!("{} missiles left", missiles-ready);

}
