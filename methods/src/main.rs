struct Wallet {
    owner: String,
    balance: u32,
}

// Tutaj otwieramy blok implementacji dla naszego portfela
impl Wallet {
    // METODA 1: Tylko odczyt (niemutowalne pożyczenie)
    // Zadanie: Wypisz w konsoli tekst, np.: "Portfel gracza: [owner], stan konta: [balance] tokenów."
    fn show_info(&self) {
        println!("Portfel gracza: {} ,stan konta: {} tokenów", self.owner, self.balance);
    }

    // METODA 2: Wpłata (mutowalne pożyczenie)
    // Zadanie: Dodaj wartość 'amount' do obecnego balansu portfela.
    fn receive_funds(&mut self, amount: u32) {
        self.balance += amount
    }

    // METODA 3: Wypłata (mutowalne pożyczenie + logika)
    // Zadanie: Sprawdź, czy na koncie jest wystarczająco środków (balance >= amount).
    // Jeśli tak: odejmij 'amount' od balansu i wypisz "Wysłano [amount] tokenów.".
    // Jeśli nie: wypisz "Błąd: Niewystarczająca ilość środków!".
    fn send_funds(&mut self, amount: u32) {
            if self.balance >= amount {
                self.balance -= amount;
                println!("Wysłano {} tokenów.", amount)
            } else {
                println!("Niewystarczają ilość środków");
            }
    }
}

fn main() {
    // 1. Tworzymy nową, mutowalną instancję portfela
    let mut my_wallet = Wallet {
        owner: "Bartek".to_string(),
        balance: 100, // Zaczynamy z 100 tokenami
    };

    // 2. Testujemy metody (zobacz, jak pięknie i obiektowo to teraz wygląda!)
    my_wallet.show_info();
    
    my_wallet.receive_funds(50);
    my_wallet.show_info(); // Powinno być 150
    
    my_wallet.send_funds(200); // Powinno wywalić błąd braku środków
    
    my_wallet.send_funds(30);
    my_wallet.show_info(); // Powinno zostać 120
}