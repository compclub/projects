# Quantum Circuit Combinators

This is a (toy) combinator library for building quantum circuits.

The key to combinators is to pick a shared interface that all combinators act on and return. In this
case, it's the `QCircuit` type, which represents a quantum circuit on `n` qbits as a `2^n by 2^n`
matrix. These `QCircuits` can be combined in the following ways:

- `QCircuit::one_qbit_gate(matrix)` constructs a circuit from its (unitary!) matrix. Likewise for
  `two_qbit_gate` and `three_qbit_gate`.
- `circuit * circuit`: puts two circuits side-by-side. Joins an `n`-qbit and an `m`-qbit` circuit
  into an `n+m`-qbit circuit.
- `circuit + circuit`: puts two circuits in sequence. Runs the left one first, then the right one.
- Several builtin gates: `id`, `h`, `x`, `y`, `z`, `cz`, `ccz`.

## Running it

Install Rust with

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then run the tests with

    cargo test

or the interactive demo with

    cargo run
