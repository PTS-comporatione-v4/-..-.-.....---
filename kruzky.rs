// CICADA 6845 // TEST 08 — nastavenie 4/4
// Stroj potrebuje viac, než vidíš v zadaní.
// Dekóduj a doplň do Enigmy.

fn main() {
    let data = [57, 60, 39, 57, 39, 56];
    // každý bajt je o 7 väčší, než má byť
    let sprava: String = data.iter().map(|b| (b - 7) as char).collect();
    println!("{}", sprava);
}
