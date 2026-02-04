use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

const BEZIER_CIRCLE_COEFF: f32 = 0.5522847498307935;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerlineGlyph {
    RightTriangle,
    LeftTriangle,
    RightRounded,
    LeftRounded,
}

impl PowerlineGlyph {
    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            '\u{E0B0}' => Some(Self::RightTriangle),
            '\u{E0B2}' => Some(Self::LeftTriangle),
            '\u{E0B4}' => Some(Self::RightRounded),
            '\u{E0B6}' => Some(Self::LeftRounded),
            _ => None,
        }
    }
}

pub fn render_powerline_glyph(glyph: PowerlineGlyph, width: u32, height: u32) -> Option<Pixmap> {
    match glyph {
        PowerlineGlyph::RightTriangle => render_right_triangle(width, height),
        PowerlineGlyph::LeftTriangle => render_left_triangle(width, height),
        PowerlineGlyph::RightRounded => render_right_rounded(width, height),
        PowerlineGlyph::LeftRounded => render_left_rounded(width, height),
    }
}

fn render_right_triangle(width: u32, height: u32) -> Option<Pixmap> {
    let mut pixmap = Pixmap::new(width, height)?;
    let mut pb = PathBuilder::new();

    let w = width as f32;
    let h = height as f32;

    pb.move_to(0.0, 0.0);
    pb.line_to(w, h / 2.0);
    pb.line_to(0.0, h);
    pb.close();

    let path = pb.finish()?;

    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    Some(pixmap)
}

fn render_left_triangle(width: u32, height: u32) -> Option<Pixmap> {
    let mut pixmap = Pixmap::new(width, height)?;
    let mut pb = PathBuilder::new();

    let w = width as f32;
    let h = height as f32;

    pb.move_to(w, 0.0);
    pb.line_to(0.0, h / 2.0);
    pb.line_to(w, h);
    pb.close();

    let path = pb.finish()?;

    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    Some(pixmap)
}

fn render_right_rounded(width: u32, height: u32) -> Option<Pixmap> {
    let mut pixmap = Pixmap::new(width, height)?;
    let mut pb = PathBuilder::new();

    let w = width as f32;
    let h = height as f32;
    let radius = w.min(h / 2.0);
    let c = BEZIER_CIRCLE_COEFF;

    pb.move_to(0.0, 0.0);

    pb.cubic_to(radius * c, 0.0, radius, radius - radius * c, radius, radius);

    pb.line_to(radius, h - radius);

    pb.cubic_to(radius, h - radius + radius * c, radius * c, h, 0.0, h);

    pb.close();

    let path = pb.finish()?;

    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    Some(pixmap)
}

fn render_left_rounded(width: u32, height: u32) -> Option<Pixmap> {
    let mut pixmap = Pixmap::new(width, height)?;
    let mut pb = PathBuilder::new();

    let w = width as f32;
    let h = height as f32;
    let radius = w.min(h / 2.0);
    let c = BEZIER_CIRCLE_COEFF;

    pb.move_to(w, 0.0);

    pb.cubic_to(
        w - radius * c,
        0.0,
        w - radius,
        radius - radius * c,
        w - radius,
        radius,
    );

    pb.line_to(w - radius, h - radius);

    pb.cubic_to(w - radius, h - radius + radius * c, w - radius * c, h, w, h);

    pb.close();

    let path = pb.finish()?;

    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;

    pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
    );

    Some(pixmap)
}

pub struct PowerlineAtlas {
    pub(crate) textures: std::collections::HashMap<PowerlineGlyph, wgpu::Texture>,
    pub(crate) views: std::collections::HashMap<PowerlineGlyph, wgpu::TextureView>,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) bind_group: Option<wgpu::BindGroup>,
}

impl PowerlineAtlas {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cell_width: f32,
        cell_height: f32,
    ) -> Self {
        let mut textures = std::collections::HashMap::new();
        let mut views = std::collections::HashMap::new();

        let width = cell_width.ceil() as u32;
        let height = cell_height.ceil() as u32;

        let glyphs = [
            PowerlineGlyph::RightTriangle,
            PowerlineGlyph::LeftTriangle,
            PowerlineGlyph::RightRounded,
            PowerlineGlyph::LeftRounded,
        ];

        for glyph in glyphs {
            if let Some(pixmap) = render_powerline_glyph(glyph, width, height) {
                let texture = device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("Powerline Glyph"),
                    size: wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                });

                queue.write_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &texture,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    pixmap.data(),
                    wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(4 * width),
                        rows_per_image: Some(height),
                    },
                    wgpu::Extent3d {
                        width,
                        height,
                        depth_or_array_layers: 1,
                    },
                );

                let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
                textures.insert(glyph, texture);
                views.insert(glyph, view);
            }
        }

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Powerline Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            textures,
            views,
            sampler,
            bind_group: None,
        }
    }
}
