use super::tile::Tile;

#[derive(Debug, Clone)]
pub struct Background {
    pub lines: Vec<BackgroundLine>,
}
impl Default for Background {
    fn default() -> Self {
        Background {
            lines: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundCell {
    pub tile: Tile,
    pub palette_value: [u8; 4],
    pub scroll_x: u16,
    pub scroll_y: u16,
    pub is_visible: bool,
}
pub type BackgroundLine = Vec<BackgroundCell>;

