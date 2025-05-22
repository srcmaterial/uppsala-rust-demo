use anyhow::{Result, Context};
use graphviz_rust::cmd::{Format, Layout};
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::exec;
use image::{DynamicImage, GenericImageView, ImageBuffer};
use std::path::Path;
use crate::graph::RdfGraph;

/// Visualization configuration
pub struct VisualizationConfig {
    pub layout: Layout,
    pub node_style: NodeStyle,
    pub edge_style: EdgeStyle,
    pub graph_attrs: Vec<(String, String)>,
}

/// Node styling options
pub struct NodeStyle {
    pub shape: String,
    pub style: String,
    pub color: String,
    pub fontname: String,
    pub fontsize: String,
}

/// Edge styling options
pub struct EdgeStyle {
    pub style: String,
    pub color: String,
    pub fontname: String,
    pub fontsize: String,
    pub arrowhead: String,
}

/// Returns a visualization configuration based on the selected preset
pub fn get_preset_config(preset: crate::VisualizationPreset) -> VisualizationConfig {
    match preset {
        crate::VisualizationPreset::Standard => standard_preset(),
        crate::VisualizationPreset::Hierarchical => hierarchical_preset(),
        crate::VisualizationPreset::Clustered => clustered_preset(),
    }
}

/// Standard visualization preset
fn standard_preset() -> VisualizationConfig {
    VisualizationConfig {
        layout: Layout::Dot,
        node_style: NodeStyle {
            shape: "ellipse".to_string(),
            style: "filled".to_string(),
            color: "#4285F4".to_string(),
            fontname: "Arial".to_string(),
            fontsize: "12".to_string(),
        },
        edge_style: EdgeStyle {
            style: "solid".to_string(),
            color: "#666666".to_string(),
            fontname: "Arial".to_string(),
            fontsize: "10".to_string(),
            arrowhead: "normal".to_string(),
        },
        graph_attrs: vec![
            ("bgcolor".to_string(), "white".to_string()),
            ("rankdir".to_string(), "LR".to_string()),
            ("splines".to_string(), "true".to_string()),
        ],
    }
}

/// Hierarchical visualization preset
fn hierarchical_preset() -> VisualizationConfig {
    VisualizationConfig {
        layout: Layout::Dot,
        node_style: NodeStyle {
            shape: "box".to_string(),
            style: "filled".to_string(),
            color: "#34A853".to_string(),
            fontname: "Arial".to_string(),
            fontsize: "12".to_string(),
        },
        edge_style: EdgeStyle {
            style: "solid".to_string(),
            color: "#666666".to_string(),
            fontname: "Arial".to_string(),
            fontsize: "10".to_string(),
            arrowhead: "vee".to_string(),
        },
        graph_attrs: vec![
            ("bgcolor".to_string(), "white".to_string()),
            ("rankdir".to_string(), "TB".to_string()),
            ("nodesep".to_string(), "0.5".to_string()),
            ("ranksep".to_string(), "1.0".to_string()),
        ],
    }
}

/// Clustered visualization preset
fn clustered_preset() -> VisualizationConfig {
    VisualizationConfig {
        layout: Layout::Fdp,
        node_style: NodeStyle {
            shape: "circle".to_string(),
            style: "filled".to_string(),
            color: "#FBBC05".to_string(),
            fontname: "Arial".to_string(),
            fontsize: "12".to_string(),
        },
        edge_style: EdgeStyle {
            style: "dashed".to_string(),
            color: "#EA4335".to_string(),
            fontname: "Arial".to_string(),
            fontsize: "10".to_string(),
            arrowhead: "open".to_string(),
        },
        graph_attrs: vec![
            ("bgcolor".to_string(), "white".to_string()),
            ("overlap".to_string(), "false".to_string()),
            ("splines".to_string(), "curved".to_string()),
            ("K".to_string(), "0.6".to_string()),
        ],
    }
}

/// Renders a graph as an image and saves it to the specified path
pub fn render_graph(graph: &RdfGraph, output_path: &Path, config: &VisualizationConfig) -> Result<()> {
    let dot_graph = create_dot_graph(graph, config);
    
    // Convert dot graph to dot string
    let dot_string = dot_string(&dot_graph);
    
    // Render the graph to PNG
    let output = exec(dot_string, config.layout, Format::Png)
        .with_context(|| "Failed to render graph")?;
    
    // Write the PNG to file
    std::fs::write(output_path, output)
        .with_context(|| format!("Failed to write image to {}", output_path.display()))?;
    
    Ok(())
}

/// Creates a dot graph from an RDF graph
fn create_dot_graph(graph: &RdfGraph, config: &VisualizationConfig) -> Graph {
    let mut dot_graph = graph!(strict di id!("RDFGraph"));
    
    // Add graph attributes
    for (key, value) in &config.graph_attrs {
        dot_graph.add_stmt(stmt!(attr!(key, value)));
    }
    
    // Add node attributes
    let node_attrs = vec![
        attr!("shape", &config.node_style.shape),
        attr!("style", &config.node_style.style),
        attr!("color", &config.node_style.color),
        attr!("fontname", &config.node_style.fontname),
        attr!("fontsize", &config.node_style.fontsize),
    ];
    dot_graph.add_stmt(stmt!(node_attrs));
    
    // Add edge attributes
    let edge_attrs = vec![
        attr!("style", &config.edge_style.style),
        attr!("color", &config.edge_style.color),
        attr!("fontname", &config.edge_style.fontname),
        attr!("fontsize", &config.edge_style.fontsize),
        attr!("arrowhead", &config.edge_style.arrowhead),
    ];
    dot_graph.add_stmt(stmt!(edge_attrs));
    
    // Add nodes
    for (node_idx, label) in &graph.node_labels {
        let node_id = format!("N{}", node_idx.index());
        dot_graph.add_stmt(stmt!(node!(node_id; attr!("label", label))));
    }
    
    // Add edges
    for (edge, label) in &graph.edge_labels {
        let source_id = format!("N{}", edge.0.index());
        let target_id = format!("N{}", edge.1.index());
        dot_graph.add_stmt(stmt!(edge!(source_id => target_id; attr!("label", label))));
    }
    
    dot_graph
}