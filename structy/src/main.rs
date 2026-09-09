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