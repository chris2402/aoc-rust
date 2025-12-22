use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    str::{FromStr, Lines},
};

use anyhow::anyhow;
use petgraph::{acyclic::Acyclic, prelude::DiGraphMap};

pub struct Manifold {
    graph: Acyclic<DiGraphMap<ManifoldNode, usize>>,
}

impl Manifold {
    pub fn new(graph: Acyclic<DiGraphMap<ManifoldNode, usize>>) -> Self {
        Manifold { graph }
    }
    pub fn _node_count(&self) -> usize {
        self.graph.node_count()
    }

    fn _nodes(&self) -> impl Iterator<Item = ManifoldNode> + Clone {
        self.graph.nodes()
    }

    pub fn _edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn count_worlds(self) -> usize {
        let start_node = self
            .graph
            .nodes()
            .find(|n| n.is_start())
            .expect("No start-node found");

        self.calculate_worlds_recursive_memoized(start_node, &mut HashMap::new())
    }

    fn calculate_worlds_recursive_memoized(
        &self,
        node: ManifoldNode,
        memoized: &mut HashMap<ManifoldNode, usize>,
    ) -> usize {
        if let Some(cached) = memoized.get(&node) {
            return *cached;
        }

        if node.is_end() {
            return 1;
        }

        let mut total_options = 0;
        for neighbor in self
            .graph
            .neighbors_directed(node, petgraph::Direction::Outgoing)
        {
            total_options += self.calculate_worlds_recursive_memoized(neighbor, memoized);
        }

        memoized.insert(node, total_options);
        total_options
    }

    pub fn split_count(&self) -> usize {
        self.graph
            .all_edges()
            .map(|e| e.1)
            .filter(|edge| edge.is_splitter())
            .collect::<HashSet<_>>()
            .len()
    }
}

impl FromStr for Manifold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: DiGraphMap<ManifoldNode, usize> = DiGraphMap::new();
        let mut active_lanes = HashMap::<usize, Vec<ManifoldNode>>::new();

        let mut lines = s.trim().lines();

        let start_node = try_parse_start_line(&mut lines)?;
        graph.add_node(start_node);
        active_lanes.insert(start_node.x(), vec![start_node]); // Start node sends flow down its lane

        for node in manifold_nodes(lines)
            .map(ManifoldNode::from)
            .filter(|n| n.is_splitter())
        {
            // Add node to graph
            graph.add_node(node);

            // Check if tachyon is in lane
            if let Some(upstream_nodes) = active_lanes.remove(&node.x()) {
                // Hit - connect to upstream nodes
                for upstream_node in upstream_nodes {
                    graph.add_edge(upstream_node, node, node.y() - upstream_node.y());
                }
                // Add new active lanes
                let (left, right) = node.get_neighbour_lanes();
                if let Some(left) = left {
                    active_lanes.entry(left).or_default().push(node);
                }
                if let Some(right) = right {
                    active_lanes.entry(right).or_default().push(node);
                }
            }
        }

        // Create end-node

        // Connect all tachyon paths to end-node
        for (x, nodes) in active_lanes {
            let end_node = ManifoldNode::new(x, usize::MAX, Space::End);
            graph.add_node(end_node);
            for upstream_node in nodes {
                graph.add_edge(upstream_node, end_node, usize::MAX - upstream_node.y());
            }
        }

        Ok(Manifold::new(
            Acyclic::try_from_graph(graph).map_err(|e| anyhow!("Graph contains cycle: {:?}", e))?,
        ))
    }
}

fn try_parse_start_line(lines: &mut Lines<'_>) -> Result<ManifoldNode, <Manifold as FromStr>::Err> {
    let start_pos = lines
        .next()
        .and_then(|line| line.chars().position(|c| c == 'S'))
        .ok_or(anyhow!("Missing start position"))?;

    let start_node = ManifoldNode::new(start_pos, 0, Space::Start);

    Ok(start_node)
}

fn manifold_nodes<'a>(lines: Lines<'a>) -> impl Iterator<Item = (usize, usize, Space)> + 'a {
    lines.enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter_map(move |(x, c)| {
            let space = Space::try_from(c).ok()?;
            Some((x, y + 1, space))
        })
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ManifoldNode {
    x: usize,
    y: usize,
    space: Space,
}

impl ManifoldNode {
    fn new(x: usize, y: usize, space: Space) -> Self {
        ManifoldNode { x, y, space }
    }

    fn is_splitter(&self) -> bool {
        self.space == Space::Splitter
    }

    fn is_start(&self) -> bool {
        self.space == Space::Start
    }

    fn is_end(&self) -> bool {
        self.space == Space::End
    }

    fn get_neighbour_lanes(&self) -> (Option<usize>, Option<usize>) {
        (self.x.checked_sub(1), self.x.checked_add(1))
    }

    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl From<(usize, usize, Space)> for ManifoldNode {
    fn from((x, y, space): (usize, usize, Space)) -> Self {
        ManifoldNode { x, y, space }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Space {
    Splitter,
    Free,
    Start,
    End,
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '^' => Ok(Space::Splitter),
            '.' => Ok(Space::Free),
            'S' => Ok(Space::Start),
            _ => Err(anyhow!("Unknown space character: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = " .......S.......
...............
.......^.......
...............
......^.^......
...............
";
    #[test]
    fn it_parses_all_nodes() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;
        assert_eq!(manifold._node_count(), 7);
        Ok(())
    }

    #[test]
    fn it_parses_split_nodes() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;
        assert_eq!(manifold._nodes().filter(|n| n.is_splitter()).count(), 3);
        Ok(())
    }

    #[test]
    fn it_parses_end_nodes() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;

        let end_nodes = manifold._nodes().filter(|n| n.is_end());
        assert_eq!(end_nodes.clone().count(), 3);
        Ok(())
    }

    #[test]
    fn it_parses_start_node() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;
        assert_eq!(manifold._nodes().filter(|n| n.is_start()).count(), 1);
        Ok(())
    }
    #[test]
    fn it_parses_all_edges() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;
        assert_eq!(manifold._edge_count(), 7);
        Ok(())
    }

    #[test]
    fn it_parses_worlds() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;
        assert_eq!(manifold.count_worlds(), 4);
        Ok(())
    }

    #[test]
    fn it_parses_splits() -> Result<(), anyhow::Error> {
        let manifold: Manifold = INPUT.parse()?;
        assert_eq!(manifold.split_count(), 3);
        Ok(())
    }
}
