use std::any::Any;

use benihime_renderer::Renderer;

use crate::{
    application::Application,
    editor_state::EditorState,
    graphics::{CursorKind, Rect},
    keymap::key_chord::KeyChord,
    position::Position,
    ui::job::Jobs,
};

pub type Callback = Box<dyn FnOnce(&mut Composer, &mut Context)>;
pub type SyncCallback = Box<dyn FnOnce(&mut Composer, &mut Context) + Sync>;

pub enum EventResult {
    Ignored(Option<Callback>),
    Consumed(Option<Callback>),
}

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyChord),
    Mouse(benihime_renderer::event::MouseEvent),
    Scroll(benihime_renderer::event::ScrollDelta),
    Resize(u16, u16),
    IdleTimeout,
    FocusGained,
    FocusLost,
}

pub type Surface = Renderer;

pub struct Context<'a> {
    pub editor: &'a mut EditorState,
    pub scroll: Option<usize>,
    pub jobs: &'a mut Jobs,
    pub dt: f32,
}

impl Context<'_> {
    pub fn block_try_flush_writes(&mut self) -> anyhow::Result<()> {
        {
            let editor = &mut *self.editor;
            let jobs = &mut *self.jobs;
            tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(jobs.finish(editor, None))
            })?;
        }

        {
            let editor = &mut *self.editor;
            tokio::task::block_in_place(move || {
                tokio::runtime::Handle::current().block_on(editor.flush_writes())
            })?
        }

        Ok(())
    }
}

pub trait Component: Any + AnyComponent {
    fn handle_event(&mut self, _event: &Event, _ctx: &mut Context) -> EventResult {
        EventResult::Ignored(None)
    }

    fn should_update(&self) -> bool {
        true
    }

    fn render(&mut self, area: Rect, surface: &mut Surface, ctx: &mut Context);

    fn cursor(&self, _area: Rect, _ctx: &Application) -> (Option<Position>, CursorKind) {
        (None, CursorKind::Hidden)
    }

    fn required_size(&mut self, _viewport: (u16, u16)) -> Option<(u16, u16)> {
        None
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn id(&self) -> Option<&'static str> {
        None
    }

    fn is_animating(&self) -> bool {
        false
    }
}

pub struct Composer {
    pub layers: Vec<Box<dyn Component>>,
    area: Rect,

    pub(crate) last_picker: Option<Box<dyn Component>>,
    pub(crate) full_redraw: bool,
}

impl Composer {
    pub fn new(area: Rect) -> Self {
        Self {
            layers: Vec::new(),
            area,
            last_picker: None,
            full_redraw: false,
        }
    }

    pub fn size(&self) -> Rect {
        self.area
    }

    pub fn resize(&mut self, area: Rect) {
        self.area = area;
    }

    pub fn push(&mut self, mut layer: Box<dyn Component>) {
        let size = self.size();
        layer.required_size((size.width, size.height));
        self.layers.push(layer);
    }

    pub fn replace_or_push<T: Component>(&mut self, id: &'static str, layer: T) {
        if let Some(component) = self.find_id(id) {
            *component = layer;
        } else {
            self.push(Box::new(layer))
        }
    }

    pub fn pop(&mut self) -> Option<Box<dyn Component>> {
        self.layers.pop()
    }

    pub fn remove(&mut self, id: &'static str) -> Option<Box<dyn Component>> {
        let idx = self
            .layers
            .iter()
            .position(|layer| layer.id() == Some(id))?;
        Some(self.layers.remove(idx))
    }

    pub fn handle_event(&mut self, event: &Event, cx: &mut Context) -> bool {
        let mut callbacks = Vec::new();
        let mut consumed = false;

        for layer in self.layers.iter_mut().rev() {
            match layer.handle_event(event, cx) {
                EventResult::Consumed(Some(callback)) => {
                    callbacks.push(callback);
                    consumed = true;
                    break;
                }
                EventResult::Consumed(None) => {
                    consumed = true;
                    break;
                }
                EventResult::Ignored(Some(callback)) => {
                    callbacks.push(callback);
                }
                EventResult::Ignored(None) => {}
            };
        }

        for callback in callbacks {
            callback(self, cx)
        }

        consumed
    }

    pub fn render(&mut self, area: Rect, surface: &mut Surface, cx: &mut Context) {
        for layer in &mut self.layers {
            layer.render(area, surface, cx);
        }
    }

    pub fn cursor(&self, area: Rect, editor: &Application) -> (Option<Position>, CursorKind) {
        for layer in self.layers.iter().rev() {
            if let (Some(pos), kind) = layer.cursor(area, editor) {
                return (Some(pos), kind);
            }
        }
        (None, CursorKind::Hidden)
    }

    pub fn has_component(&self, type_name: &str) -> bool {
        self.layers
            .iter()
            .any(|component| component.type_name() == type_name)
    }

    pub fn find<T: 'static>(&mut self) -> Option<&mut T> {
        let type_name = std::any::type_name::<T>();
        self.layers
            .iter_mut()
            .find(|component| component.type_name() == type_name)
            .and_then(|component| component.as_any_mut().downcast_mut())
    }

    pub fn find_id<T: 'static>(&mut self, id: &'static str) -> Option<&mut T> {
        self.layers
            .iter_mut()
            .find(|component| component.id() == Some(id))
            .and_then(|component| component.as_any_mut().downcast_mut())
    }

    pub fn need_full_redraw(&mut self) {
        self.full_redraw = true;
    }
}

pub trait AnyComponent {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Component> AnyComponent for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl dyn AnyComponent {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }

    pub fn downcast<T: Any>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        if self.as_any().is::<T>() {
            Ok(self.as_boxed_any().downcast().unwrap())
        } else {
            Err(self)
        }
    }

    pub fn is<T: Any>(&mut self) -> bool {
        self.as_any().is::<T>()
    }
}
