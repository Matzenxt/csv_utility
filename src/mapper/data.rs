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