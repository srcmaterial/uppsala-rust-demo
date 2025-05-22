# RDF-Viz

A Rust tool for converting RDF/XML files to network visualizations.

## Features

- Parse RDF/XML files
- Convert RDF triples to a network graph
- Visualize the graph with three preset styles:
  - Standard: Basic layout with elliptical nodes
  - Hierarchical: Top-to-bottom layout emphasizing structure
  - Clustered: Force-directed layout grouping related nodes
- Export to PNG image

## Requirements

- Rust (2018 edition or later)
- Graphviz (for graph rendering)

## Installation

1. Ensure that Graphviz is installed on your system:
   - macOS: `brew install graphviz`
   - Ubuntu/Debian: `apt-get install graphviz`
   - Windows: Download from [Graphviz website](https://graphviz.org/download/)

2. Clone this repository:
   ```
   git clone https://github.com/yourusername/rdf-viz.git
   cd rdf-viz
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Usage

```
rdf-viz --input input.rdf --output graph.png [--preset PRESET]
```

### Options

- `-i, --input <FILE>`: Input RDF/XML file (required)
- `-o, --output <FILE>`: Output image file (required)
- `-p, --preset <PRESET>`: Visualization preset [standard, hierarchical, clustered] (default: standard)
- `-h, --help`: Print help
- `-V, --version`: Print version

## Examples

```
# Generate visualization with standard preset
rdf-viz --input data.rdf --output visualization.png

# Generate visualization with hierarchical layout
rdf-viz --input data.rdf --output visualization.png --preset hierarchical

# Generate visualization with clustered layout
rdf-viz --input data.rdf --output visualization.png --preset clustered
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.