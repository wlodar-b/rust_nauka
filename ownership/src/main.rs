
/* 
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}


fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}"); // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
    */

fn main() {
    // 1. Inicjalizujemy status palety. 
    // Dzięki 'mut' ta zmienna dostaje pełny pakiet uprawnień: +R (odczyt), +W (zapis), +O (własność).
    let mut status_palety = String::from("W PRZYJMOWANIU"); 
        
    // 3. W międzyczasie magazynier klika na skanerze, że paleta jest gotowa.
    // Próbujemy dopisać tekst do oryginalnej zmiennej:
    status_palety.push_str(" - ZAKOŃCZONE"); // ❌ TUTAJ KOMPILATOR UDERZA W STÓŁ!

    // 2. Moduł raportujący WMS-a chce tylko podejrzeć status, więc go WYPOŻYCZAMY.
    let raport = &status_palety; 


    // 4. Moduł raportujący drukuje swój podgląd.
    println!("Dzienny raport: {}", raport);
}