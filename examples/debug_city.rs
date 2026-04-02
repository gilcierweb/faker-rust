use faker::address;
use faker::name;

fn main() {
    println!("Testing address::city() placeholder resolution");
    println!("city(): {}", address::city());
}
