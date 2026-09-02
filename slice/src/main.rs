/*
fn main() {
    let mut s = String::from("hello");
    let hello: &str = &s[0..5]; // s traci W i O na czas życia hello

    println!("{hello}");

    s.push_str(" world"); // OK dopiero teraz, bo hello już nie jest używane
}
*/

fn reszta_zdania(tekst: &str) -> &str {
    let bajty = tekst.as_bytes(); // 1. Zamień tekst na bajty

    for (i, &item) in bajty.iter().enumerate() {
        if item == b' ' { // 2. Przeiteruj po nich szukając spacji
            return &tekst[(i + 1)..]; // 3. Zwróć wycinek od (i + 1) do końca
        }
    }  ""// 4. Zwróć "" jeśli nie ma spacji
}

fn main() {
    let powitanie = String::from("Witajcie rdzawi programiści");
    let reszta = reszta_zdania(&powitanie);
    
    println!("Tekst bez pierwszego słowa: {}", reszta); // Powinno wypisać: "rdzawi programiści"
}