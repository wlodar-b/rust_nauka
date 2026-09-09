struct Product {
    name: String,
    price: f64,
    in_stock: bool,
}

fn main() {
    let klawiatura = Product {
        name: String::from("Klawiatura"),
        price: 99.99,
        in_stock: true,
    };

println!("Dostępny produkt to: {} w cenie: {} zł", klawiatura.name, klawiatura.price);
}