fn main() {
    let mut korytarz_a = vec!["Paleta 1", "Paleta 2"];
    
    // System zapisuje referencję do pierwszej palety
    let pierwsza_paleta = &korytarz_a[0]; 
    
    // Wjeżdża nowa dostawa na koniec korytarza
    korytarz_a.push("Paleta 3"); // ❌ Tu kompilator rzuca błędem!
    
    // System drukuje raport
    println!("Na froncie korytarza wciąż stoi: {}", pierwsza_paleta); 
}