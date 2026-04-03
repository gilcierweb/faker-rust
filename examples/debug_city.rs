use faker_rust::address;
use faker_rust::name;

fn main() {
    println!("Testing address::city() placeholder resolution");
    println!("city(): {}", address::city());
}
