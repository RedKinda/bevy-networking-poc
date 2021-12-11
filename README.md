# Bevy networking proof of concept

Proof of concept for a simple networking logic, using bevy and https://github.com/smokku/bevy_networking_turbulence

Product vision:  build a client and a server. Server sends game world updates, client renders. Acceptance criteria: 2
clients connect to the server (on localhost). Each client operates a dot, wherever client clicks, dot moves with flat
speed to the point clicked. Both clients render updated positions of dots.

## How to run

`cargo run` opens a new desktop window serving as a client

`cargo make build` in the `client/` folder will open a server serving at `http://127.0.0.1:4000/` with the client
compiled to WASM, visible as a canvas on the page. Note that you need to `cargo install cargo-make` beforehand.