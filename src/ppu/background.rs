use super::tile::Tile;

#[derive(Debug, Clone)]
pub struct Background {
    pub lines: Vec<BackgroundLine>,
    pub scroll_x: u8,
    pub scroll_y: u8,
    pub mirroring: Mirroring,
}
impl Default for Background {
    fn default() -> Self {
        Background {
            lines: Vec::new(),
            scroll_x: 0,
            scroll_y: 0,
            mirroring: Mirroring::Horizontal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackgroundCell {
    pub tile: Tile,
    pub palette_value: [u8; 4],
}
pub type BackgroundLine = Vec<BackgroundCell>;

#[derive(Debug, Clone)]
pub enum Mirroring {
    Horizontal,
    Vertical,
}
