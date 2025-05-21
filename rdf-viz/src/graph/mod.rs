use anyhow::Result;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

/// Represents an RDF graph as a directed graph
pub struct RdfGraph {
    pub graph: DiGraph<String, String>,
    pub node_labels: HashMap<NodeIndex, String>,
    pub edge_labels: HashMap<(NodeIndex, NodeIndex), String>,
    pub node_map: HashMap<String, NodeIndex>,
}

impl RdfGraph {
    pub fn new() -> Self {
        RdfGraph {
            graph: DiGraph::new(),
            node_labels: HashMap::new(),
            edge_labels: HashMap::new(),
            node_map: HashMap::new(),
        }
    }

    /// Adds a node to the graph if it doesn't exist yet
    pub fn add_node(&mut self, uri: &str) -> NodeIndex {
        if let Some(&node_idx) = self.node_map.get(uri) {
            return node_idx;
        }

        // Shorten labels for better visualization
        let label = get_shortened_label(uri);
        
        let node_idx = self.graph.add_node(label.clone());
        self.node_labels.insert(node_idx, label);
        self.node_map.insert(uri.to_string(), node_idx);
        
        node_idx
    }

    /// Adds an edge between two nodes
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, predicate: &str) {
        // Skip adding duplicate edges with the same predicate
        if self.edge_labels.iter().any(|(&(s, t), p)| s == from && t == to && p == predicate) {
            return;
        }
        
        // Shorten predicate for better visualization
        let label = get_shortened_label(predicate);
        
        let edge = self.graph.add_edge(from, to, label.clone());
        self.edge_labels.insert((from, to), label);
    }
}

/// Builds a graph from RDF triples
pub fn build_graph(triples: Vec<(String, String, String)>) -> Result<RdfGraph> {
    let mut graph = RdfGraph::new();
    
    for (subject, predicate, object) in triples {
        let subject_idx = graph.add_node(&subject);
        let object_idx = graph.add_node(&object);
        
        graph.add_edge(subject_idx, object_idx, &predicate);
    }
    
    Ok(graph)
}

/// Shortens a URI to its last component or a readable label
fn get_shortened_label(uri: &str) -> String {
    // Extract the last part of the URI after # or last /
    if let Some(fragment_pos) = uri.rfind('#') {
        uri[(fragment_pos + 1)..].to_string()
    } else if let Some(path_pos) = uri.rfind('/') {
        uri[(path_pos + 1)..].to_string()
    } else {
        // If it's a literal or simple value
        let max_len = 30;
        if uri.len() > max_len {
            format!("{}...", &uri[0..max_len])
        } else {
            uri.to_string()
        }
    }
}