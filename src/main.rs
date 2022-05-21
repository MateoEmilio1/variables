fn main() {
    let missiles = 8;
    let ready = 2;
    println!(" Firing {} of my {} ... ", ready, missiles);
    let missiles = missiles - ready;
    println! ("{} missiles left", missiles);
}
