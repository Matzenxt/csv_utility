use std::fmt;
use serde::{Deserialize, Serialize};
use serde_json::Result;





#[derive(Serialize, Deserialize)]
pub struct Mappings {
    pub mappings: Vec<Map>
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub dest_entry: HeaderEntry,
    pub source_entry: Option<HeaderEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderEntry {
    pub name: String,
    pub position: usize,
}

impl Mappings {
    pub fn new(dest_headers: Vec<String>) -> Mappings {
        let mut header_mappings: Vec<Map> = Vec::new();

        for (index, dest_header) in dest_headers.iter().enumerate() {
            header_mappings.push(Map::new(index, dest_header.to_string()));
        };

        Mappings {
            mappings: header_mappings
        }
    }
}

impl Map {
    pub fn new(dest_pos: usize, dest_name: String) -> Map {
        Map {
            dest_entry: HeaderEntry {
                name: dest_name,
                position: dest_pos
            },
            source_entry: None
        }
    }

    pub fn set_source_entry(&mut self, new_source_entry: Option<HeaderEntry>) {
        self.source_entry = new_source_entry;
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dest_text: String = match &self.source_entry {
            Some(header_entry) => {
                header_entry.name.to_string()
            }
            None => {
                "Empty".to_string()
            }
        };

        write!(f, "{} <- {}", self.dest_entry.name, dest_text)
    }
}