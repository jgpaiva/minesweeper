# Minesweeper in the browser

This project is a minesweeper built with [rust](https://www.rust-lang.org/), [wasm](https://webassembly.org/) and [yew](https://yew.rs/). It started with the goal of becoming a complete solver for minesweeper in the CLI, but ended up becoming a complete game + basic solver library with a browser and a CLI frontend.

## Demo

You can play the WASM version online [here](https://jgpaiva.github.io/minesweeper/).

## How to run localy

After installing rust and cargo, run `wasm-pack build --dev --target web`. After
this, start a server on the local folder (e.g. `python3 -m http.server`) and
check it out on your favourite browser.

## Example output

![demo output](imgs/demo.png)
