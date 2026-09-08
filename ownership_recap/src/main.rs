/* 
fn main() {
    let n = 0; 
    n += 1; // Błąd!
}

// NAPRAWIONY KOD 1:
fn main() {
    let mut n = 0; // Teraz `n` ma uprawnienia: +R, +W, +O
    n += 1;        // Działa bez problemu!
    println!("{}", n);
}
*/

fn main() {
    let s = String::from("Hello world");
    consume_a_string(s); 
    println!("{s}"); // Błąd!
}

fn consume_a_string(_s: String) {
    // om nom nom
}