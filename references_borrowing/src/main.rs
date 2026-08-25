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
        log_systemowy.push_str(" [PILNE]");  
    } else {
        // Jeśli to nie był błąd, oznaczamy jako zwykłe INFO
        log_systemowy.push_str(" [INFO]");   
    }
}
*/
/*
fn main() {
    let mut wozek = String::from("Wózek nr 3: W spoczynku");

    // 1. System dyspozytora pobiera referencję, żeby tylko odczytać status
    let podglad = &wozek;

    
    // 3. Dyspozytor odświeża ekran i odczytuje swój podgląd
    println!("Na ekranie dyspozytora: {}", podglad);

    // 2. Operator rusza wózkiem i system próbuje zmienić status.
    // Używamy &mut, żeby funkcja mogła fizycznie zmienić oryginalny tekst!
    zmien_status(&mut wozek, "W ruchu"); 

}

// Funkcja, która przyjmuje referencję mutowalną i podmienia tekst
fn zmien_status(maszyna: &mut String, nowy_status: &str) {
    maszyna.clear();
    maszyna.push_str(nowy_status);
}
*/

/*
fn main() {
    let mut polecenie = String::from("Rozładunek rampy nr 1");

            // 1. Pracownik A patrzy na swój terminal (odczyt)
    let ekran_pracownika_a = &polecenie;
    println!("Pracownik A poszedł wykonać: {}", ekran_pracownika_a);

    // 2. Kierownik nagle zmienia priorytet w systemie (wymaga &mut)
    zmien_priorytet(&mut polecenie, "PILNE: Załadunek tira nr 4"); 

    // 3. Pracownik B patrzy na swój terminal (odczyt)
    let ekran_pracownika_b = &polecenie;

    // 4. System loguje to, co zobaczyli pracownicy
    println!("Pracownik B poszedł wykonać: {}", ekran_pracownika_b);
}

// Funkcja kierownika
fn zmien_priorytet(aktualne_polecenie: &mut String, nowe: &str) {
    aktualne_polecenie.clear();
    aktualne_polecenie.push_str(nowe);
}
*/

/*
// 1. Zmieniamy sygnaturę funkcji: zwracamy pełnoprawny String, a nie referencję (&String)
fn wygeneruj_kod() -> String {
    let nowy_kod = String::from("KOD-9988");
    // 2. Oddajemy samą zmienną, BEZ ampersanda. Przenosimy jej własność wyżej.
    nowy_kod 
}

fn main() {
    // 3. Zmienna 'kod' staje się nowym, dumnym właścicielem danych na Stercie!
    let kod = wygeneruj_kod();
    println!("Skanuję: {}", kod);
}
*/

fn main() {
    let paleta = String::from("Paleta nr 12 - Elektronika");
    
    let inspekcja = &paleta; // 1. Wypożyczamy do odczytu
    
    println!("Raport z inspekcji: {}", inspekcja); // 3. Ostatnie użycie referencji
    
    drop(paleta); 
    
}