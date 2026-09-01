// ZMIEŃ TEN KOD:
fn get_first(name: &String) -> &String {
    &name
}

fn main() {
    let mut name = (
        String::from("Ferris"),
        String::from("Rustacean")
    );
    
    // ZMIEŃ TO WYWOŁANIE:
    let first = get_first(&name.0); 
    
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}