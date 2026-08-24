/* fn main() {
    let towar = String::from("Laptop Poleasingowy Lenovo");
    
    // Zamiast oddawać, WYPOŻYCZAMY (&) wskaźnik do towaru
    wydrukuj_etykiete(&towar); 
    
    // Zmienna 'towar' wciąż jest nasza, więc możemy wypożyczyć ją ponownie!
    zapisz_do_bazy(&towar); 
}

// Funkcja musi wiedzieć, że dostaje tylko referencję (&String), a nie wartość na własność
fn wydrukuj_etykiete(nazwa: &String) {
    println!("Drukuję etykietę dla: {}", nazwa);
}

fn zapisz_do_bazy(nazwa: &String) {
    println!("Zapisano w bazie: {}", nazwa);
}
 */

fn main() {
    let mut korytarz_a = vec!["Paleta 1", "Paleta 2"];
    
    // System zapisuje referencję do pierwszej palety
    let pierwsza_paleta = &korytarz_a[0]; 
    

    // System drukuje raport
    println!("Na froncie korytarza wciąż stoi: {}", pierwsza_paleta); 

    // Wjeżdża nowa dostawa na koniec korytarza
    korytarz_a.push("Paleta 3"); // ❌ Tu kompilator rzuca błędem!
    
}