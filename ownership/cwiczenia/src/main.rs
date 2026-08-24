fn main() {
    let towar = String::from("Laptop Poleasingowy Lenovo");
    
    let klon_towaru = towar.clone();
    wydrukuj_etykiete(towar);
    zapisz_do_bazy(klon_towaru); // ❌ Tu kompilator rzuca błędem!
}

fn wydrukuj_etykiete(nazwa: String) {
    println!("Drukuję etykietę dla: {}", nazwa);
}

fn zapisz_do_bazy(nazwa: String) {
    println!("Zapisano w bazie: {}", nazwa);
}