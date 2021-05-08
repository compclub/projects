use jp_quantum::QCircuit;
use std::collections::HashMap;
use std::io;
use std::io::Write;

const INTRO: &str = "Specify a circuit, column by column.
Each column lists its gates from top to bottom.
Columns are separated by `;`s, and gates are separated by spaces.
Each column must be the same size; pad with `id` gates if you need to.
The supported gates are:

Gate|Qbits|Description
----+-----+-----------
id  |  1  | Identity
h   |  1  | Hadamard
x   |  1  | Pauli X
y   |  1  | Pauli Y
z   |  1  | Pauli Z
cz  |  2  | Controlled Z (control bits are first)
czz |  3  | Doubly-controlled Z
";

fn read_line_from_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");
    buffer
}

fn gates_table() -> HashMap<&'static str, QCircuit> {
    use jp_quantum::gates::*;

    let mut table = HashMap::new();
    table.insert("id", id());
    table.insert("h", h());
    table.insert("x", x());
    table.insert("y", y());
    table.insert("z", z());
    table.insert("cz", cz());
    table.insert("ccz", ccz());
    table
}

fn build_circuit(circuit_str: &str) -> QCircuit {
    let gates_table = gates_table();
    let mut circuit = None;
    for column_str in circuit_str.split(";") {
        if column_str.trim() == "" {
            continue;
        }
        let mut column = QCircuit::empty();
        for gate_str in column_str.split(" ").filter(|s| *s != "") {
            let gate = gates_table
                .get(gate_str.trim())
                .unwrap_or_else(|| {
                    panic!("Unrecognized gate '{}'", gate_str);
                })
                .clone();
            column = column * gate;
        }
        circuit = match circuit {
            None => Some(column),
            Some(circuit) => Some(circuit + column),
        };
    }
    circuit.expect("Circuit was empty")
}

fn main() {
    println!("{}", INTRO);
    loop {
        println!("Enter a circuit, or nothing to quit:");
        println!();
        print!("> ");
        io::stdout().flush().unwrap();
        let circuit_str = read_line_from_stdin();
        if circuit_str.trim() == "" {
            break;
        }
        let circuit = build_circuit(&circuit_str);
        let distribution = circuit.run().measure();
        println!("Samples:");
        for _ in 0..5 {
            print!("  ");
            for _ in 0..5 {
                print!("{:<12}", distribution.sample());
            }
            println!();
        }
        println!();
    }
}
