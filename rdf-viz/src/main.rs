use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

mod rdf;
mod graph;
mod visualization;

#[derive(Parser)]
#[command(
    name = "rdf-viz",
    about = "Converts RDF/XML files to network visualizations",
    version,
    author,
)]
struct Cli {
    /// Input RDF/XML file
    #[arg(short, long)]
    input: PathBuf,

    /// Output image file
    #[arg(short, long)]
    output: PathBuf,

    /// Visualization preset
    #[arg(short, long, value_enum, default_value_t = VisualizationPreset::Standard)]
    preset: VisualizationPreset,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum VisualizationPreset {
    /// Standard visualization with basic layout
    Standard,
    
    /// Hierarchical visualization emphasizing structure
    Hierarchical,
    
    /// Cluster-based visualization grouping related nodes
    Clustered,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Parse RDF file
    let triples = rdf::parse_rdf_xml(&cli.input)?;
    
    // Convert to graph
    let graph = graph::build_graph(triples)?;
    
    // Render visualization
    let viz_config = visualization::get_preset_config(cli.preset);
    visualization::render_graph(&graph, &cli.output, &viz_config)?;
    
    println!("Visualization created: {}", cli.output.display());
    Ok(())
}