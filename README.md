# puzzle
15 puzzle game solver using A* algorithm with manhattan distance heuristic and disjoint databases heuristic.

## Setup
You need cargo to build and run this program.
You can install it using rustup: https://rustup.rs/

To run this project locally, compile it using cargo:
```
cargo build --release
````

## Code examples
Create random instance of 15 puzzle game and solve it using disjoint databases heuristic.
```
cargo run --release --bin puzzle -- --heuristic disjoint-databases
```

Create random instance of 15 puzzle game and solve it using manhattan distance heuristic.
```
cargo run --release --bin puzzle -- --heuristic manhattan-distance
```
