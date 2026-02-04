use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use glyphon::{Buffer, FontSystem, Metrics, Shaping};

#[derive(Clone, Debug)]
pub struct ShapedTextKey {
    pub text: String,
    pub metrics: (u32, u32),
    pub color: [u8; 4],
}

impl Hash for ShapedTextKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.metrics.hash(state);
        self.color.hash(state);
    }
}

impl PartialEq for ShapedTextKey {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.metrics == other.metrics && self.color == other.color
    }
}

impl Eq for ShapedTextKey {}

pub struct CachedShapedText {
    pub buffer: Buffer,
    pub last_used_frame: u64,
    pub generation: u64,
}

pub struct ShapedTextCache {
    pub entries: HashMap<ShapedTextKey, CachedShapedText>,
    pub current_frame: u64,
    pub current_generation: u64,
    max_entries: usize,
    pub hits: u64,
    pub misses: u64,
}

impl ShapedTextCache {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(max_entries / 2),
            current_frame: 0,
            current_generation: 0,
            max_entries,
            hits: 0,
            misses: 0,
        }
    }

    pub fn get_or_shape(
        &mut self,
        key: ShapedTextKey,
        font_system: &mut FontSystem,
        metrics: Metrics,
        width: f32,
        height: f32,
    ) -> &mut Buffer {
        if !self.entries.contains_key(&key) {
            self.misses += 1;

            let mut buffer = Buffer::new(font_system, metrics);
            buffer.set_size(font_system, Some(width), Some(height));

            use glyphon::{Attrs, Family};
            let attrs = Attrs::new().family(Family::SansSerif).metrics(metrics);

            buffer.set_text(font_system, &key.text, &attrs, Shaping::Advanced, None);
            buffer.shape_until_scroll(font_system, false);

            if self.entries.len() >= self.max_entries {
                self.evict_lru();
            }

            let entry = CachedShapedText {
                buffer,
                last_used_frame: self.current_frame,
                generation: self.current_generation,
            };

            self.entries.insert(key.clone(), entry);
        } else {
            self.hits += 1;
        }

        let entry = self.entries.get_mut(&key).unwrap();
        entry.last_used_frame = self.current_frame;

        entry
            .buffer
            .set_size(font_system, Some(width), Some(height));

        &mut entry.buffer
    }

    pub fn next_frame(&mut self) {
        self.current_frame += 1;

        if self.current_frame.is_multiple_of(60) {
            self.cleanup_stale_entries();
        }
    }

    pub fn evict_lru(&mut self) {
        if let Some((key, _)) = self
            .entries
            .iter()
            .min_by_key(|(_, entry)| entry.last_used_frame)
            .map(|(k, e)| (k.clone(), e.last_used_frame))
        {
            self.entries.remove(&key);
        }
    }

    fn cleanup_stale_entries(&mut self) {
        let stale_threshold = self.current_frame.saturating_sub(300);
        self.entries
            .retain(|_, entry| entry.last_used_frame > stale_threshold);
    }

    pub fn invalidate(&mut self) {
        self.current_generation += 1;
        self.entries.clear();
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
}
