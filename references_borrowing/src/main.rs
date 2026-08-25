 /* fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
} */

/*

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
*/

/* 
fn main() {
    let mut log_systemowy = String::from("Blad: Brak towaru na regale A12");

    // Wypożyczamy do odczytu pierwsze 4 znaki z logu
    let fragment = &log_systemowy[..4]; 

    if fragment == "Blad" {
        println!("UWAGA! Wykryto: {}", fragment);
        // Zauważyliśmy błąd, więc chcemy dopisać flagę PILNE do oryginału
        log_systemowy.push_str(" [PILNE]");  // ❌ TUTAJ KOMPILATOR RZUCA BŁĘDEM (brak uprawnienia W)
    } else {
        // Jeśli to nie był błąd, oznaczamy jako zwykłe INFO
        log_systemowy.push_str(" [INFO]");   // ✅ TUTAJ KOD ZADZIAŁA BEZ PROBLEMU!
    }
}
*/

