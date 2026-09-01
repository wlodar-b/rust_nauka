fn main() {
    let mut koszyk: Vec<(String, u32, u32)> = vec![
        (String::from("Chleb"), 5, 2),
        (String::from("Mleko"), 4, 3),
        (String::from("Baton"), 1, 100)
    ];

    wypisz_paragon(&koszyk);
    let do_zaplaty = oblicz_sume(&koszyk);
    println!("Całkowita kwota do zapłaty wynosi: {}zł", do_zaplaty);
}

fn wypisz_paragon(lista: &Vec<(String, u32, u32)>) {
    for (nazwa, cena, ilosc) in lista {
        println!("{} - {}szt. x {}zł", nazwa, ilosc, cena);
    }
}

fn oblicz_sume(produkty: &Vec<(String, u32, u32)>) -> u32 {
    let mut kwota = 0;

    for (_nazwa, cena, ilosc) in produkty {
    kwota += cena * ilosc;
    }
    kwota
}