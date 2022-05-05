mod renderer;
mod schema;
mod solver;

use schema::*;
use solver::SolvedCircuit;

fn main() {
    let solution = SolvedCircuit::from(Circuit::new(
        9.0,
        Node::Parallel(vec![
            Node::Series(vec![
                Node::Resistor(10.0),
                Node::Ammeter("A"),
                Node::Resistor(20.0),
            ]),
            Node::Voltameter("V"),
        ]),
    ));
    println!("{:#?}", solution);
}

#[cfg(test)]
mod tests {
    use crate::{Circuit, Node, SolvedCircuit};

    #[test]
    fn complex_example() {
        let solution = SolvedCircuit::from(Circuit::new(
            9.0,
            Node::Series(vec![
                Node::Series(vec![
                    Node::Resistor(10.0),
                    Node::Ammeter("A"),
                    Node::Resistor(20.0),
                ]),
                Node::Parallel(vec![
                    Node::Parallel(vec![Node::Resistor(20.0), Node::Resistor(10.0)]),
                    Node::Resistor(10.0),
                ]),
            ]),
        ));
        println!("{}", solution);
    }

    #[test]
    fn simple_parallel_circuit() {
        let solution = SolvedCircuit::from(Circuit::new(
            9.0,
            Node::Parallel(vec![
                Node::Series(vec![
                    Node::Resistor(10.0),
                    Node::Ammeter("A"),
                    Node::Resistor(20.0),
                ]),
                Node::Voltameter("V"),
            ]),
        ));
        println!("{}", solution);
    }

    #[test]
    fn simple_series_circuit() {
        let solution = SolvedCircuit::from(Circuit::new(
            9.0,
            Node::Series(vec![Node::Resistor(10.0), Node::Resistor(20.0)]),
        ));
        println!("{}", solution);
    }

    #[test]
    fn top_level_resistor() {
        let solution = SolvedCircuit::from(Circuit::new(5.0, Node::Resistor(10.0)));
        println!("{}", solution);
    }

    #[test]
    fn top_level_voltameter() {
        let solution = SolvedCircuit::from(Circuit::new(12.0, Node::Voltameter("lable")));
        println!("{}", solution);
    }
}
