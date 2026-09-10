/* 
struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}


fn build_product(name: String, price: f64) -> Product {
    Product {
        name,
        price,
        in_stock: true,
    }
}

fn main() {
    let mut klawiatura = build_product(String::from("Klawiatura mechaniczna"), 299.99);

println!("Dostępny produkt to: {} w cenie: {} zł", klawiatura.name, klawiatura.price);
klawiatura.in_stock = false;
println!("Czy produkt jest dostępny? {}", klawiatura.in_stock);
} 
*/
/*
struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}

fn main() {
    let klawiatura_v1 = Product {
        name: String::from("Klawiatura mechaniczna"),
        price: 350.00,
        in_stock: true,
    };

    println!("Oryginalna nazwa: {}", klawiatura_v1.name);

    // Tworzymy nową, tańszą wersję używając skrótu:
    let klawiatura_v2 = Product {
        price: 299.99,
        ..klawiatura_v1
    };

    println!("Oryginalna cena: {}", klawiatura_v1.price);
}

*/

/*
struct Size(u32, u32);
struct Coordinates(u32, u32);

fn main() {
    let size = Size(800, 600);
    let coordinate = Coordinates(10, 20);
    // let pozycja: Coordinates = size;


    println!("Szerokość struktury Size wynosi: {}", size.0);

    let Coordinates(x, y) = coordinate;
    println!("Współrzędne to: ({}, {})", x, y);
}
*/
/*
struct ZdarzenieZapisano;

fn main() {
    let sygnal = ZdarzenieZapisano;

    println!("Zdarzenie zostało poprawnie obsłuzone!");
}
*/

/*
struct Post {
    title: String,
    content: String,
    likes: u32,
}

fn main() {
    let moj_post = Post {
        title: "Mój pierwszy post".to_string(),
        content: "Uczę się Rusta i idzie mi świetnie!".to_string(),
        likes: 10,
    };
}
*/
/*
struct Player {
    name: String,
    score: u32,
}

fn main() {
    let mut gracz = Player {
        name: String::from("Hero"),
        score: 10,
    };

    // Pożyczamy TYLKO wynik mutowalnie
    let points_ref = &mut gracz.score;

    // PYTANIE 1: Czy ta linijka się skompiluje, jeśli usuniemy komentarz?
    println!("Imię gracza to: {}", gracz.name);

    // Zwiększamy wynik przez referencję
    *points_ref += 5;

    // PYTANIE 2: Czy usunięcie komentarza z poniższej linijki (gdybyśmy wstawili 
    // ją ZANIM `points_ref` przestanie być używane) wywołałoby błąd?
    println!("Cały gracz to: {}, punktów: {}", gracz.name, gracz.score);
}
*/


struct Point {
  x: i32,
  y: i32,
}
fn main() {
  let mut a = Point { x: 1, y: 2 };
  a.x += 1;
  let b = Point { y: 1, ..a };
  a.x += 1;
  println!("{}", b.x);
}