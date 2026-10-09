# CICADA 6845 // TEST 08 — nastavenie 1/4
# Stroj potrebuje viac, než vidíš v zadaní.
# Dekóduj a doplň do Enigmy.
import base64

signal = "VUtXIDIgMQ=="  # base64

print(base64.b64decode(signal).decode("utf-8"))
