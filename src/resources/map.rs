use std::collections::HashMap;
use std::fs;

// 中身も公開する
pub struct Map {
    pub layout: String,
    pub tiles: HashMap<(usize, usize), String>,
    pub width: usize,
    pub height: usize,
}

// Map構造体のコンストラクタのような動作をする
// setupで実行する
impl Default for Map {
    fn default() -> Self {
        // ファイル内容全体を文字列として読み込む
        match fs::read_to_string("assets/config/map_layout.txt") {
            Ok(layout) => {
                let mut tiles = HashMap::new();

                let mut width: usize = 0;
                let mut height: usize = 0;

                // 先頭と末尾の空白を削除して、改行ごとにインデックス付きで取り出す
                for (y, line) in layout.trim().lines().enumerate(){
                    height += 1;
                    let mut line_width = 0;

                    //1行を列ごとに分割する
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