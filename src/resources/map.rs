use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Map {
    pub layout: String,
    pub tiles: HashMap<(usize, usize), String>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn read_layout() -> String {
        let path = Path::new("assets/config/map_layout.txt");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();

        if let Err(why) = file.read_to_string(&mut s) {
            panic!("couldn't read {}: {}", display, why)
        }

        String::from(s.trim())
    }
}

impl Default for Map {
    fn default() -> Self {
        let layout = Self::read_layout();

        let mut tiles = HashMap::new();

        let mut width: usize = 0;
        let mut height: usize = 0;

        for (y, line) in layout.lines().enumerate() {
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
}
