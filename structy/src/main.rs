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