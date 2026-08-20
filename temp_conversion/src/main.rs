use std::io;

fn main() {

    let mut wybor = String::new();    
   
    println!("Kalkulator konwersji temperatur.");
    println!("1. Celsjusze na Fahrenheity");
    println!("2. Fahrenheity na Celsjusze");
    println!("Wybierz co chcesz przeliczyć: ");

    io::stdin()
        .read_line(&mut wybor)
        .expect("Failed to read line");


    let wybor: u32 = wybor
        .trim()
        .parse()
        .expect("Prosze wpisac cyfre 1 lub 2!");

    match wybor {
        1 => {
            c_to_f();
        }
        2 => {
            f_to_c()
        }
        _ => {
            println!("Nie ma takiej opcji w menu");
        }
    }

}


fn c_to_f() {
    println!("Podaj temperature do przeliczenia na fahrenheity");
    print!("Temperatura: ");

    let mut temp_wejsciowa_c = String::new();

    io::stdin()
        .read_line(&mut temp_wejsciowa_c)
        .expect("Podaj temperature");

    let temp_wejsciowa_c: f64 = temp_wejsciowa_c
        .trim()
        .parse()
        .expect("Prosze wpisac cyfre.");

    let f = temp_wejsciowa_c * 1.8 + 32.0;

    println!("Podana temperatura w przeliczeniu na Fahrenheity wynosi: {}", f);
}

fn f_to_c() {
    println!("Podaj temperature do przeliczenia na Celsjusze");
    print!("Temperatura: ");

    let mut temp_wejsciowa_f = String::new();

    io::stdin()
        .read_line(&mut temp_wejsciowa_f)
        .expect("Podaaj temperature");

    let temp_wejsciowa_f: f64 = temp_wejsciowa_f
        .trim()
        .parse()
        .expect("Prosze wpisac cyfre.");

    let c = (temp_wejsciowa_f - 32.0) / 1.8;

    println!("Podana temperatura w przeliczeniu na Celsjusze wynosi: {:.2}", c);
}