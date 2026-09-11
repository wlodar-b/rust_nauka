struct Wallet {
    owner: String,
    balance: u32,
}

impl Wallet {
    // METODA 1: Tylko odczyt (niemutowalne pożyczenie)
    fn show_info(&self) {
        println!("Portfel gracza: {} ,stan konta: {} tokenów", self.owner, self.balance);
    }

    // METODA 2: Wpłata (mutowalne pożyczenie)
    fn receive_funds(&mut self, amount: u32) {
        self.balance += amount
    }

    // METODA 3: Wypłata (mutowalne pożyczenie + logika)
    fn send_funds(&mut self, amount: u32) {
            if self.balance >= amount {
                self.balance -= amount;
                println!("Wysłano {} tokenów.", amount)
            } else {
                println!("Niewystarczają ilość środków");
            }
    }


// METODA 4: Przelew do innego portfela
    fn transfer(&mut self, receiver: &mut Wallet, amount: u32) {
        if self.balance >= amount {
            self.balance -= amount;
            receiver.balance += amount;
            println!("Przelano {} tokenów od {} do {}", amount, self.owner, receiver.owner);
        }   else {
            println!("Błąd: Niewystarczająca ilość środków na przelew!");
        }
    }

    fn new(owner_name: String, initial_balance: u32) -> Self {
        Self {
            owner: owner_name,
            balance: initial_balance,
        }
    }
}
fn main() {
    let mut my_wallet = Wallet::new("Bartek".to_string(), 100);
    let mut alice_wallet = Wallet::new("Alice".to_string(), 100);

    println!("Stan początkowy portfeli:");
    my_wallet.show_info();
    alice_wallet.show_info();
    
    println!("Próba przelewu środków na konto alice");
    my_wallet.transfer(&mut alice_wallet, 30);

    println!("Stan po przelewie:");
    my_wallet.show_info();
    alice_wallet.show_info();
}