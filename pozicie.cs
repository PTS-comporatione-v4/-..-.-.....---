// CICADA 6845 // TEST 08 — nastavenie 3/4
// Stroj potrebuje viac, než vidíš v zadaní.
// Dekóduj a doplň do Enigmy.
using System;
using System.Linq;

class Pozicie {
    static void Main() {
        var data = new[]{7, 22, 13};
        // každé číslo je o 5 väčšie, než má byť
        Console.WriteLine(string.Join(" ", data.Select(x => x - 5)));
    }
}
