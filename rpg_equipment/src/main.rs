struct Item {
    name: String,
    weight: f64,
    value: u32,
}

struct Player {
    name: String,
    hp: u32,
    inventory: Vec<Item>,
}

fn main() {
    let mut geralt = Player {
        name: "Geralt".to_string(),
        hp: 100,
        inventory: Vec::new(),
    };

    let miecz = Item {
        name: "Nithing".to_string(),
        weight: 3.5,
        value: 100,
    };

    let zbroja = Item {
        name: "Kołczan".to_string(),
        weight: 9.7,
        value: 200,
    };

    let kusza = Item {
        name: "Super_Kusza".to_string(),
        weight: 1.2,
        value: 30,
    };

    loot_item(&mut geralt, miecz);
    loot_item(&mut geralt, zbroja);
    loot_item(&mut geralt, kusza);
    show_inventory(&geralt);
    calculate_total_weight(&geralt);
    println!("Aktualne HP {} wynosi: {}", geralt.name, geralt.hp );
    take_damage(&mut geralt, 30);
    println!("Aktualne HP {} wynosi: {}", geralt.name, geralt.hp );
}

fn take_damage(player: &mut Player, amount: u32) {
    if player.hp > amount {
        player.hp -= amount;
    } else { 
        player.hp = 0;
    } 
}

fn loot_item(player: &mut Player, item: Item) {
    player.inventory.push(item);
}

fn calculate_total_weight(player: &Player) -> f64 {
    let mut total_weight = 0.0;
    for item in &player.inventory {
        total_weight += item.weight;
    }
    println!("Waga całego ekwipunku wynosi: {:.1}kg.", total_weight);
    total_weight
}

fn show_inventory(player: &Player) {
    if player.inventory.is_empty() {
        println!("Ekwipunek jest pusty.");
    } else {
        println!("Lista przedmiotów w ekwipunku: ");
        for item in &player.inventory {
            println!("- {} ({} kg) - Wartość: {}", item.name, item.weight, item.value);
        }
    }
}
