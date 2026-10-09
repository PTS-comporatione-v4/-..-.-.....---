// CICADA 6845 // TEST 08 — nastavenie 2/4
// Stroj potrebuje viac, než vidíš v zadaní.
// Dekóduj a doplň do Enigmy.

const signal = "492049492049";  // šestnástková sústava

const sprava = Buffer.from(signal, "hex").toString("utf-8");
console.log(sprava);
