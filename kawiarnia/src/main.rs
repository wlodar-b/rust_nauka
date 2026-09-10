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
    show_queue(&moja_kawiarnia);
    serve_next_order(&mut moja_kawiarnia);
    show_queue(&moja_kawiarnia);
    serve_next_order(&mut moja_kawiarnia);
    show_queue(&moja_kawiarnia);
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

fn serve_next_order(cafe: &mut Cafe) {
    if cafe.queue.is_empty() {
        println!("Brak zamówień do obsłużenia.");
    } else {
        let gotowe_zamowienie = cafe.queue.remove(0);
        
        println!("Wydano: {} dla {} (Zamówienie #{})", gotowe_zamowienie.drink, gotowe_zamowienie.customer, gotowe_zamowienie.id);
    }
}

fn show_queue(cafe: &Cafe) {
    if cafe.queue.is_empty() {
        println!("Kolejka jest pusta.");
    } else {
        for order in &cafe.queue {
            println!("Zamówienie: {} dla: {} (Zamówienie #{})", order.drink, order.customer, order.id);
        }
    }
}