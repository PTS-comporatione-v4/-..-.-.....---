// CICADA 6845 // TEST 08 — bonus
// Toto nie je súčasť testu. Len ozvena.
// Dekóduj, ak chceš.

fn main() {
    let data = [91, 80, 74, 79, 86, 39, 79, 86, 93, 86, 89, 80, 39, 85, 72, 81, 79, 83, 72, 90, 85, 76, 81, 90, 80, 76];
    // každý bajt je o 7 väčší, než má byť
    let sprava: String = data.iter().map(|b| (b - 7) as char).collect();
    println!("{}", sprava);
}
