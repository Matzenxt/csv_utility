// TODO: Add data structure for mapping

use std::borrow::BorrowMut;

pub struct HeaderMap {
    maps: Vec<Map>,
}

pub struct Map {
    dest_entry: HeaderEntry,
    source_entry: Option<HeaderEntry>,
}

pub struct HeaderEntry {
    name: String,
    position: usize,
}

impl HeaderMap {
    pub fn new(dest_headers: Vec<String>) -> HeaderMap {
        let mut temp: Vec<Map> = vec![];

        let mut pos_counter: usize = 0;
        for dest_header in dest_headers {
            temp.push(Map::new(pos_counter, dest_header));
        }

        HeaderMap {
            maps: temp,
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
}