# wl-transit

Wl-tranit ist eine simple Browser-Page, weiters gedacht als [PWA](https://de.wikipedia.org/wiki/Progressive_Web_App), mit der man alle Abfahrtzeiten von Öffis der [Wiener Linien](https://www.wienerlinien.at/) in einem bestimmten Radius auf einen Blick sehen kann.

![Example Image](https://raw.githubusercontent.com/hapqe/wl-transit/main/examples/wl-transit.png)

# Verwendung
Die Seite ist unter [](https://bit.ly/wltr) einsehbar

# Building
Stelle sicher, dass du `bun`, `cargo`, `wasm-bindgen` und `wasm-opt` installiert hast. Zuerst wechsle in das Verzeichnis `station_picker` mit `cd` und führe `./bindgen.sh` aus. Anschließend führe `bun` run build aus. Alternativ kannst du auch `npm` verwenden.

# Credit

Datenquelle: Stadt Wien - https://data.wien.gv.at
