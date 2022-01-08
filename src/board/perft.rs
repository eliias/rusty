use std::fmt::{Display, Formatter};
use crate::{Board, Color};

#[derive(Copy, Clone)]
pub struct PerftResult {
    pub(crate) depth: usize,
    pub(crate) nodes: usize,
    pub(crate) captures: usize,
    pub(crate) ep: usize,
    pub(crate) castles: usize,
    pub(crate) promotions: usize,
    pub(crate) checks: usize,
    pub(crate) discovery_checks: usize,
    pub(crate) double_checks: usize,
    pub(crate) checkmates: usize,
}

impl Default for PerftResult {
    fn default() -> Self {
        PerftResult {
            depth: 0,
            nodes: 0,
            captures: 0,
            ep: 0,
            castles: 0,
            promotions: 0,
            checks: 0,
            discovery_checks: 0,
            double_checks: 0,
            checkmates: 0,
        }
    }
}

pub trait Perft {
    fn perft(&self, depth: usize) -> PerftResult;
    fn _perft(&self, depth: usize, board: Board, color: Color, result: PerftResult) -> PerftResult;

    fn divide(&self, max_depth: usize) -> PerftResult;
}

impl Display for PerftResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let data =
            [
                format!("Nodes: {}", self.nodes),
                format!("Captures: {}", self.captures),
                format!("E.p.: {}", self.ep),
                format!("Castles: {}", self.castles),
                format!("Promotions: {}", self.promotions),
                format!("Checks: {}", self.checks),
                format!("Discovery checks: {}", self.discovery_checks),
                format!("Double checks: {}", self.double_checks),
                format!("Checkmates: {}", self.checkmates),
            ];
        write!(f, "{}", data.join("\n"))
    }
}
