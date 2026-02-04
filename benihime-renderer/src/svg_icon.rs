use std::collections::HashMap;

use resvg::tiny_skia::Pixmap;

#[derive(Clone)]
pub struct RasterizedIcon {
    pub pixmap: Pixmap,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct IconCacheKey {
    svg_hash: u64,
    width: u32,
    height: u32,
}

pub struct SvgIconCache {
    cache: HashMap<IconCacheKey, RasterizedIcon>,
    svg_data: HashMap<u64, Vec<u8>>,
}

impl SvgIconCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            svg_data: HashMap::new(),
        }
    }

    fn hash_svg(data: &[u8]) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    pub fn rasterize(
        &mut self,
        svg_data: &[u8],
        width: u32,
        height: u32,
    ) -> Option<&RasterizedIcon> {
        let hash = Self::hash_svg(svg_data);
        let key = IconCacheKey {
            svg_hash: hash,
            width,
            height,
        };

        if self.cache.contains_key(&key) {
            return self.cache.get(&key);
        }

        if !self.svg_data.contains_key(&hash) {
            self.svg_data.insert(hash, svg_data.to_vec());
        }

        let icon = render_svg(svg_data, width, height)?;
        self.cache.insert(key.clone(), icon);
        self.cache.get(&key)
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.svg_data.clear();
    }

    pub fn stats(&self) -> (usize, usize) {
        (self.cache.len(), self.svg_data.len())
    }
}

impl Default for SvgIconCache {
    fn default() -> Self {
        Self::new()
    }
}

pub fn render_svg(svg_data: &[u8], width: u32, height: u32) -> Option<RasterizedIcon> {
    if width == 0 || height == 0 {
        return None;
    }

    let tree = resvg::usvg::Tree::from_data(svg_data, &resvg::usvg::Options::default()).ok()?;

    let svg_size = tree.size();
    let svg_width = svg_size.width();
    let svg_height = svg_size.height();

    if svg_width <= 0.0 || svg_height <= 0.0 {
        return None;
    }

    let scale_x = width as f32 / svg_width;
    let scale_y = height as f32 / svg_height;
    let scale = scale_x.min(scale_y);

    let rendered_width = (svg_width * scale).ceil() as u32;
    let rendered_height = (svg_height * scale).ceil() as u32;

    let mut pixmap = Pixmap::new(width, height)?;

    let offset_x = (width as f32 - rendered_width as f32) / 2.0;
    let offset_y = (height as f32 - rendered_height as f32) / 2.0;

    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_translate(offset_x, offset_y).post_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    Some(RasterizedIcon {
        pixmap,
        width,
        height,
    })
}

pub fn render_svg_with_color(
    svg_data: &[u8],
    width: u32,
    height: u32,
    color: (u8, u8, u8, u8),
) -> Option<RasterizedIcon> {
    let mut icon = render_svg(svg_data, width, height)?;

    let data = icon.pixmap.data_mut();
    for chunk in data.chunks_exact_mut(4) {
        let alpha = chunk[3];
        if alpha > 0 {
            chunk[0] = color.0;
            chunk[1] = color.1;
            chunk[2] = color.2;
            chunk[3] = ((alpha as u16 * color.3 as u16) / 255) as u8;
        }
    }

    Some(icon)
}
