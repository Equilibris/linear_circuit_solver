use std::fmt::{Display, Write};

use crate::{renderer::scientific_float_renderer, schema::*};
#[derive(Debug, Clone)]
pub struct Data {
    voltage: f64,
    amp: f64,

    resistance: f64,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = format!(
            "{}, {}, {}",
            scientific_float_renderer(self.amp, "A"),
            scientific_float_renderer(self.voltage, "V"),
            scientific_float_renderer(self.resistance, "Ω")
        );
        f.write_str(v.as_str())
    }
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
pub enum SolvedNode {
    Series(Data, Vec<SolvedNode>),

    Parallel(Data, Vec<SolvedNode>),

    Resistor(Data),
    Voltameter(Data, &'static str),
    Ammeter(Data, &'static str),
}

impl Display for SolvedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            SolvedNode::Series(data, values) => format!(
                "Series[{}]( {} )",
                data,
                values
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            SolvedNode::Parallel(data, values) => format!(
                "Parallel[{}]( {} )",
                data,
                values
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            SolvedNode::Resistor(data) => format!("R[{}]", data),
            SolvedNode::Voltameter(data, label) => format!(
                "Volt#{}={}",
                label,
                scientific_float_renderer(data.voltage, "V")
            ),
            SolvedNode::Ammeter(data, label) => {
                format!("Amp#{}={}", label, scientific_float_renderer(data.amp, "A"))
            }
        };
        f.write_str(v.as_str())
    }
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
pub struct SolvedCircuit {
    battery_voltage: f64,
    node: SolvedNode,
}

impl Display for SolvedCircuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = format!(
            "V_batery = {}: {{ {} }}",
            scientific_float_renderer(self.battery_voltage, "V"),
            self.node
        );
        f.write_str(v.as_str())
    }
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
