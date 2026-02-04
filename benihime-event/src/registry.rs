use std::any::TypeId;

use anyhow::{Result, bail};
use hashbrown::{HashMap, hash_map::Entry};
use parking_lot::RwLock;

use crate::{hook::ErasedHook, runtime_local};

pub struct Registry {
    events: HashMap<&'static str, TypeId, foldhash::fast::FixedState>,
    handlers: HashMap<&'static str, Vec<ErasedHook>, foldhash::fast::FixedState>,
}

impl Registry {
    pub fn register_event<E: Event + 'static>(&mut self) {
        let ty = TypeId::of::<E>();
        assert_eq!(ty, TypeId::of::<E::Static>());
        match self.events.entry(E::ID) {
            Entry::Occupied(entry) => {
                if entry.get() == &ty {
                    panic!("Event {} was registered multiple times", E::ID);
                } else {
                    panic!("Multiple events with ID {} were registered", E::ID);
                }
            }
            Entry::Vacant(ent) => {
                ent.insert(ty);
                self.handlers.insert(E::ID, Vec::new());
            }
        }
    }

    pub unsafe fn register_hook<E: Event>(
        &mut self,
        hook: impl Fn(&mut E) -> Result<()> + 'static + Send + Sync,
    ) {
        let id = E::ID;
        let Some(&event_id) = self.events.get(id) else {
            panic!("Tried to register handler for unknown event {id}");
        };
        assert!(
            TypeId::of::<E::Static>() == event_id,
            "Tried to register invalid hook for event {id}"
        );
        let hook = ErasedHook::new(hook);
        self.handlers.get_mut(id).unwrap().push(hook);
    }

    pub fn register_dynamic_hook(
        &mut self,
        hook: impl Fn() -> Result<()> + 'static + Send + Sync,
        id: &str,
    ) -> Result<()> {
        if self.events.get(id).is_none() {
            bail!("Tried to register handler for unknown event {id}");
        };
        let hook = ErasedHook::new_dynamic(hook);
        self.handlers.get_mut(id).unwrap().push(hook);
        Ok(())
    }

    pub fn dispatch<E: Event>(&self, mut event: E) {
        let Some(hooks) = self.handlers.get(E::ID) else {
            log::error!("Dispatched unknown event {}", E::ID);
            return;
        };
        let event_id = self.events[E::ID];

        assert_eq!(
            TypeId::of::<E::Static>(),
            event_id,
            "Tried to dispatch invalid event {}",
            E::ID
        );

        for hook in hooks {
            if let Err(err) = unsafe { hook.call(&mut event) } {
                log::error!("{} hook failed: {err:#?}", E::ID);
                crate::status::report_blocking(err);
            }
        }
    }
}

runtime_local! {
    static REGISTRY: RwLock<Registry> = RwLock::new(Registry {
        events: HashMap::with_hasher(foldhash::fast::FixedState::with_seed(72536814787)),
        handlers: HashMap::with_hasher(foldhash::fast::FixedState::with_seed(72536814787)),
    });
}

pub(crate) fn with<T>(f: impl FnOnce(&Registry) -> T) -> T {
    f(&REGISTRY.read())
}

pub(crate) fn with_mut<T>(f: impl FnOnce(&mut Registry) -> T) -> T {
    f(&mut REGISTRY.write())
}

pub unsafe trait Event: Sized {
    const ID: &'static str;
    const LIFETIMES: usize;
    type Static: Event + 'static;
}
