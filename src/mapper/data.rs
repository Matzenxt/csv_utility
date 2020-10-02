use std::fmt;

pub struct Map {
    pub dest_entry: HeaderEntry,
    pub source_entry: Option<HeaderEntry>,
}

pub struct HeaderEntry {
    pub name: String,
    pub position: usize,
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