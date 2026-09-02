/*
fn main() {
    let mut s = String::from("hello");
    let hello: &str = &s[0..5]; // s traci W i O na czas życia hello

    println!("{hello}");

    s.push_str(" world"); // OK dopiero teraz, bo hello już nie jest używane
}
*/
/*
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
*/

fn srodek_tablicy(liczby: &[i32]) -> &[i32] {
    let dlugosc = liczby.len();
    
    if dlugosc < 3 {
        return &[]; // Zwracamy pusty wycinek, jeśli długość jest mniejsza niż 3
    }
    &liczby[1..(dlugosc - 1)]

    // if ... { return &[]; }
    // zwróć wycinek od 1 do dlugosc - 1
}

fn main() {
    let wektor_punktow = vec![10, 20, 30, 40, 50];
    let srodek_w = srodek_tablicy(&wektor_punktow);
    // Używamy {:?} do wypisywania całych wycinków/tablic
    println!("Środek wektora: {:?}", srodek_w); // Powinno wypisać: [20, 30, 40]

    let krotka_tablica = [1, 2];
    let srodek_t = srodek_tablicy(&krotka_tablica);
    println!("Środek tablicy: {:?}", srodek_t); // Powinno wypisać: []
}