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

/*
fn main() {
    let s = String::from("Hello world");
    consume_a_string(s); 
    println!("{s}"); // Błąd!
}

fn consume_a_string(_s: String) {
    // om nom nom
}
*/
/*
// NAPRAWIONY KOD 2 (Opcja A - referencje):
fn main() {
    let s = String::from("Hello world");
    
    // Przekazujemy tylko referencję. `s` zachowuje swoje uprawnienia O i W.
    consume_a_string(&s); 
    
    // `s` wciąż żyje, mamy uprawnienie R, więc to zadziała:
    println!("{s}"); 
}

fn consume_a_string(_s: &String) { // Zmiana sygnatury na referencję
    // Funkcja może tylko odczytać tekst
}

// NAPRAWIONY KOD 2 (Opcja B - klonowanie):
fn main() {
    let s = String::from("Hello world");
    
    // Tworzymy idealną kopię na stercie i oddajemy JĄ funkcji.
    // Oryginalne `s` w ogóle nie bierze udziału w przeniesieniu.
    consume_a_string(s.clone()); 
    
    println!("{s}"); // `s` zachowało wszystkie uprawnienia!
}

fn consume_a_string(_s: String) {
    // Funkcja zjada swoją własną kopię
}
*/

/*
fn main() {
    let mut s = String::from("Hello");
    let s_ref = &mut s; // ZMIANA: Tworzymy referencję mutowalną (&mut)
    
    s_ref.push_str(" world"); // Działa! *s_ref ma teraz uprawnienia +R i +W.
    println!("{s_ref}");
}
*/
/*
fn main() {
let mut s = String::from("Hello");
let s_ref = &s;
println!("{s_ref}");
s.push_str(" world");
}
*/
/*
fn main() {
let s = String::from("Hello");
let s_ref = &s;
let s2 = s_ref.clone();
println!("{s}");
}
*/
/*
fn main() {
    let mut s = String::from("Hello");
    println!("{s}");
    let s_ref = &mut s;
s_ref.push_str(" world");

}
*/

fn main() {
let mut v = vec![1, 2, 3];
let n = v[0];
v.push(4);
println!("{n}");
}