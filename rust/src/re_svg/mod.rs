pub mod data;

use crate::re_svg::data::SVGConfig;
use resvg::render;
use resvg::tiny_skia::Pixmap;
use resvg::usvg::fontdb::Database;
use resvg::usvg::{Options, Transform, Tree};
use std::sync::Arc;

pub fn parse_svg(svg_config: &SVGConfig) {
    let mut font_db = Database::new();
    font_db.load_system_fonts();
    let opt = Options {
        dpi: svg_config.dpi,
        fontdb: Arc::new(font_db),
        ..Default::default()
    };
    let tree = Tree::from_data(svg_config.str.as_bytes(), &opt).unwrap();
    let size = if svg_config.size.is_none() {
        tree.size()
    } else {
        svg_config.size.unwrap()
    };
    let mut pixel_map = Pixmap::new(size.width() as u32, size.height() as u32).unwrap();
    render(&tree, Transform::default(), &mut pixel_map.as_mut());
    if let Some(path) = &svg_config.save_path {
        pixel_map.save_png(path).unwrap()
    }
}
