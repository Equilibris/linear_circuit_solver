#[derive(Clone, Debug)]
enum Node {
    Series(Vec<Node>),
    Parallel(Vec<Node>),

    // Ohm
    Resistor(f64),
    Voltameter(&'static str),
    Ammeter(&'static str),
}

impl Node {
    fn resistance(&self) -> f64 {
        match self {
            Node::Series(v) => v.iter().map(|x| x.resistance()).sum(),
            Node::Parallel(v) => f64::recip(v.iter().map(|x| f64::recip(x.resistance())).sum()),
            Node::Resistor(v) => v.clone(),
            Node::Voltameter(_) => f64::INFINITY,
            Node::Ammeter(_) => 0.0,
        }
    }
}

#[derive(Debug)]
struct Circuit {
    battery_voltage: f64,
    node: Node,
}

impl Circuit {
    fn new(battery_voltage: f64, nodes: Node) -> Self {
        Self {
            battery_voltage,
            node: nodes,
        }
    }
}

#[derive(Debug, Clone)]
struct Data {
    voltage: f64,
    amp: f64,

    resistance: f64,
}

impl Data {
    // R = U/I
    fn from_va(voltage: f64, amp: f64) -> Data {
        Data {
            voltage,
            amp,
            resistance: voltage / amp,
        }
    }
    // I = U/R
    fn from_rv(resistance: f64, voltage: f64) -> Data {
        Data {
            voltage,
            amp: voltage / resistance,
            resistance,
        }
    }
    // U = RI
    fn from_ra(resistance: f64, amp: f64) -> Data {
        Data {
            voltage: resistance * amp,
            amp,
            resistance,
        }
    }

    fn wattage(&self) -> f64 {
        self.amp * self.voltage
    }
}

#[derive(Debug)]
enum SolvedNode {
    Series(Data, Vec<SolvedNode>),

    Parallel(Data, Vec<SolvedNode>),

    Resistor(Data),
    Voltameter(Data, &'static str),
    Ammeter(Data, &'static str),
}

impl SolvedNode {
    fn solve_with_parent(node: Node, parent: &Data) -> SolvedNode {
        match node {
            Node::Series(children) => SolvedNode::Series(
                parent.clone(),
                children
                    .into_iter()
                    .map(|node| Self::solve_with_parent(node, parent))
                    .collect(),
            ),
            Node::Parallel(children) => SolvedNode::Parallel(
                parent.clone(),
                children
                    .into_iter()
                    .map(|node| {
                        let resistance = node.resistance();
                        Self::solve_with_parent(node, &Data::from_rv(resistance, parent.voltage))
                    })
                    .collect(),
            ),
            Node::Resistor(resistance) => {
                SolvedNode::Resistor(Data::from_ra(resistance, parent.amp.clone()))
            }
            Node::Voltameter(label) => SolvedNode::Voltameter(parent.clone(), label),
            Node::Ammeter(label) => SolvedNode::Ammeter(parent.clone(), label),
        }
    }

    fn solve_entrypoint(voltage: f64, node: Node) -> SolvedNode {
        let resistance = node.resistance();

        Self::solve_with_parent(node, &Data::from_rv(resistance, voltage))
    }
}

#[derive(Debug)]
struct SolvedCircuit {
    battery_voltage: f64,
    node: SolvedNode,
}

impl From<Circuit> for SolvedCircuit {
    fn from(v: Circuit) -> Self {
        let top_level_node = v.node;
        let voltage = v.battery_voltage;

        Self {
            battery_voltage: voltage,
            node: SolvedNode::solve_entrypoint(voltage, top_level_node),
        }
    }
}

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
        println!("{:#?}", solution);
    }

    #[test]
    fn simple_series_circuit() {
        let solution = SolvedCircuit::from(Circuit::new(
            9.0,
            Node::Series(vec![Node::Resistor(10.0), Node::Resistor(20.0)]),
        ));
        println!("{:#?}", solution);
    }

    #[test]
    fn top_level_resistor() {
        let solution = SolvedCircuit::from(Circuit::new(5.0, Node::Resistor(10.0)));
        println!("{:?}", solution);
    }

    #[test]
    fn top_level_voltameter() {
        let solution = SolvedCircuit::from(Circuit::new(12.0, Node::Voltameter("lable")));
        println!("{:?}", solution);
    }
}
