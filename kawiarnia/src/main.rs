struct Order {
    id: u32,
    customer: String,
    drink: String,
}

struct Cafe {
    queue: Vec<Order>,
    next_id: u32,
}

fn main() {
let mut moja_kawiarnia = Cafe {
    queue: Vec::new(),
    next_id: 1,
};

    add_order(&mut moja_kawiarnia, "Ania".to_string(), "Latte".to_string());
    add_order(&mut moja_kawiarnia, "Basia".to_string(), "Kawa".to_string());
}

fn add_order(cafe: &mut Cafe, customer: String, drink: String) {
    
    println!("Dodano zamowienie #{}, dla klienta; {}", cafe.next_id, customer);

    let nowe_zamowienie = Order {
        id: cafe.next_id,
        customer,
        drink,
    };

    cafe.queue.push(nowe_zamowienie);

    cafe.next_id += 1;
}