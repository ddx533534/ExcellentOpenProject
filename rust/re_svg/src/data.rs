use resvg::usvg::Size;
use std::path::PathBuf;

pub struct SVGConfig {
    pub str: String, // svg 字符串
    pub dpi: f32, // 逻辑单位转换因子
    pub size: Option<Size>, // 输出图片尺寸，不传默认为svg本身大小
    pub save_path: Option<PathBuf>, // 图片保存路径
}
impl SVGConfig {
    pub fn default() -> SVGConfig {
        Self {
            str: "".to_string(),
            dpi: 96.0,
            size: None,
            save_path: None,
        }
    }
    pub fn new(str: String, dpi: f32, size: Option<Size>, save_path: Option<PathBuf>) -> Self {
        Self {
            str,
            dpi,
            size,
            save_path,
        }
    }
}
