use std::io::{self, Write};

fn main() {
    let mut ekwipunek: Vec<String> = vec![
        String::from("miecz"), 
        String::from("zbroja"), 
        String::from("hełm"), 
        String::from("buty"), 
        String::from("rekawiczki")
        ];
    
    
    println!("Witaj w systemie zarządzania ekwipunkiem!");
     
loop {
    let mut wybor = String::new();
    println!("1. Dodaj przedmiot do ekwipunku,");
    println!("2. Wyświetl ekwipunek,");
    println!("3. Zlicz wszystkie przedmoity,");
    println!("4. Wyjście z programu.");
    print!("Wybierz co chcesz zrobić: ");
    io::stdout().flush().unwrap(); 

    io::stdin() 
        .read_line(&mut wybor)
        .expect("Nie prawidłowy wybór");

    let wybor: u32 = wybor
    .trim()
    .parse()
    .expect("Wybierz cyfre 1-4!");

    match wybor {
        1 => {
            print!("Napisz co chcesz dodać do ekwipunku: ");
            io::stdout().flush().unwrap(); 
            let mut przedmiot = String::new();
            io::stdin()
                .read_line(&mut przedmiot)
                .expect("Nieprawidłowy przedmiot");
            let przedmiot = przedmiot
                .trim()
                .to_string();

            dodaj_przedmiot(&mut ekwipunek, przedmiot);
            println!("Przedmiot dodany");
        }

        2 => {
            print!("Ekwipunek składa się z: ");
            wyswietl_ekwipunek(&ekwipunek);
        }

        3 => { 
            zlicz_ekwipunek(&ekwipunek);
        }
        4 => break,
        _ => println!("Zły wybór!"),


    } 
  }
}

fn dodaj_przedmiot(lista: &mut Vec<String>, nowa_rzecz: String) {
    lista.push(nowa_rzecz);
    
}

fn wyswietl_ekwipunek(skład: &Vec<String>) {
    for element in skład {
        println!("{}", element);
    }
 
}

fn zlicz_ekwipunek(liczba: &Vec<String>) {
    println!("Liczba itemków w ekwipunku wynosi: {}", liczba.len()); 
}

