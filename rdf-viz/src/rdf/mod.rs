use anyhow::{Result, Context};
use oxrdf::{Triple, Subject, Term};
use oxrdfxml::RdfXmlParser;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Parses an RDF/XML file and returns a vector of triples in string format
/// Each triple is represented as (subject, predicate, object)
pub fn parse_rdf_xml(path: &Path) -> Result<Vec<(String, String, String)>> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open RDF file: {}", path.display()))?;
    let reader = BufReader::new(file);
    
    let parser = RdfXmlParser::new(reader);
    let triples: Result<Vec<Triple>, _> = parser.collect();
    let triples = triples.with_context(|| "Failed to parse RDF/XML")?;
    
    let string_triples = triples.into_iter()
        .map(|triple| {
            let subject = subject_to_string(&triple.subject);
            let predicate = triple.predicate.to_string();
            let object = term_to_string(&triple.object);
            
            (subject, predicate, object)
        })
        .collect();
    
    Ok(string_triples)
}

fn subject_to_string(subject: &Subject) -> String {
    match subject {
        Subject::NamedNode(node) => node.to_string(),
        Subject::BlankNode(node) => format!("_:{}", node),
        Subject::Triple(triple) => format!("<< {} {} {} >>", 
            subject_to_string(&triple.subject), 
            triple.predicate.to_string(),
            term_to_string(&triple.object))
    }
}

fn term_to_string(term: &Term) -> String {
    match term {
        Term::NamedNode(node) => node.to_string(),
        Term::BlankNode(node) => format!("_:{}", node),
        Term::Literal(literal) => literal.to_string(),
        Term::Triple(triple) => format!("<< {} {} {} >>", 
            subject_to_string(&triple.subject), 
            triple.predicate.to_string(),
            term_to_string(&triple.object))
    }
}