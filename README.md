# wl-transit

Wl-tranit ist eine simple Browser-Page, weiters gedacht als [PWA](https://de.wikipedia.org/wiki/Progressive_Web_App), mit der man alle Abfahrtzeiten von Öffis der [Wiener Linien](https://www.wienerlinien.at/) in einem bestimmten Radius auf einen Blick sehen kann.

![Example Image](https://raw.githubusercontent.com/hapqe/wl-transit/main/examples/wl-transit.png)

# Building
Make sure you have `bun`, `cargo`, `wasm-bindgen` and `wasm-opt` installed; First `cd` into `station_picker` and run `./bindgen.sh`. Then run `bun run build`. Alternatively you can use NPM.

# Credit

Datenquelle: Stadt Wien - https://data.wien.gv.at
