fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // konwersja String -> tablica bajtów

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' - literał bajtu reprezentujący spację
            return i; // znaleziono spację - zwróć jej indeks
        }
    }

    s.len() // brak spacji - całe słowo to cały string
}