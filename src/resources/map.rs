use std::collections::HashMap;
use std::fs;

pub struct Map {
    pub layout: String,
    pub tiles: HashMap<(usize, usize), String>,
    pub width: usize,
    pub height: usize,
}

impl Default for Map {
    fn default() -> Self {
        match fs::read_to_string("assets/config/map_layout.txt") {
            Ok(layout) => {
                let mut tiles = HashMap::new();

                let mut width: usize = 0;
                let mut height: usize = 0;

                for (y, line) in layout.trim().lines().enumerate() {
                    height += 1;
                    let mut line_width = 0;

                    for (x, c) in line.trim().split(' ').enumerate() {
                        line_width += 1;
                        tiles.insert((x, y), c.to_string());
                    }

                    if line_width > width {
                        width = line_width;
                    }
                }

                Map {
                    layout,
                    tiles,
                    width,
                    height,
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
