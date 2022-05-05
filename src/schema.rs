#[derive(Clone, Debug)]
pub enum Node {
    Series(Vec<Node>),
    Parallel(Vec<Node>),

    // Ohm
    Resistor(f64),
    Voltameter(&'static str),
    Ammeter(&'static str),
}

impl Node {
    pub fn resistance(&self) -> f64 {
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
pub struct Circuit {
    pub battery_voltage: f64,
    pub node: Node,
}

impl Circuit {
    pub fn new(battery_voltage: f64, nodes: Node) -> Self {
        Self {
            battery_voltage,
            node: nodes,
        }
    }
}
