use anyhow::Result;
pub use cancel::{TaskController, TaskHandle, cancelable_future};
pub use debounce::{AsyncHook, send_blocking};
pub use redraw::{
    RenderLockGuard, RequestRedrawOnDrop, lock_frame, redraw_requested, request_redraw, start_frame,
};
pub use registry::Event;

mod cancel;
mod debounce;
mod hook;
mod redraw;
mod registry;
#[doc(hidden)]
pub mod runtime;
pub mod status;

pub fn register_event<E: Event + 'static>() {
    registry::with_mut(|registry| registry.register_event::<E>())
}

pub fn register_hook_raw<E: Event>(hook: impl Fn(&mut E) -> Result<()> + 'static + Send + Sync) {
    registry::with_mut(|registry| unsafe { registry.register_hook(hook) })
}

pub fn register_dynamic_hook(
    hook: impl Fn() -> Result<()> + 'static + Send + Sync,
    id: &str,
) -> Result<()> {
    registry::with_mut(|reg| reg.register_dynamic_hook(hook, id))
}

pub fn dispatch(e: impl Event) {
    registry::with(|registry| registry.dispatch(e));
}

#[macro_export]
macro_rules! events {
    ($name: ident<$($lt: lifetime),*> { $($data:ident : $data_ty:ty),* } $($rem:tt)*) => {
        pub struct $name<$($lt),*> { $(pub $data: $data_ty),* }
        unsafe impl<$($lt),*> $crate::Event for $name<$($lt),*> {
            const ID: &'static str = stringify!($name);
            const LIFETIMES: usize = $crate::events!(@sum $(1, $lt),*);
            type Static = $crate::events!(@replace_lt $name, $('static, $lt),*);
        }
        $crate::events!{ $($rem)* }
    };
    ($name: ident { $($data:ident : $data_ty:ty),* } $($rem:tt)*) => {
        pub struct $name { $(pub $data: $data_ty),* }
        unsafe impl $crate::Event for $name {
            const ID: &'static str = stringify!($name);
            const LIFETIMES: usize = 0;
            type Static = Self;
        }
        $crate::events!{ $($rem)* }
    };
    () => {};
    (@replace_lt $name: ident, $($lt1: lifetime, $lt2: lifetime),* ) => {$name<$($lt1),*>};
    (@sum $($val: expr, $lt1: lifetime),* ) => {0 $(+ $val)*};
}

#[macro_export]
macro_rules! register_hook {
    (move |$event:ident: &mut $event_ty: ident<$($lt: lifetime),*>| $body: expr) => {
        let val = move |$event: &mut $event_ty<$($lt),*>| $body;
        unsafe {
            #[allow(unused)]
            const ASSERT: () = {
                if <$event_ty as $crate::Event>::LIFETIMES != 0 + $crate::events!(@sum $(1, $lt),*){
                    panic!("invalid type alias");
                }
            };
            $crate::register_hook_raw::<$crate::events!(@replace_lt $event_ty, $('static, $lt),*)>(val);
        }
    };
    (move |$event:ident: &mut $event_ty: ident| $body: expr) => {
        let val = move |$event: &mut $event_ty| $body;
        {
            #[allow(unused)]
            const ASSERT: () = {
                if <$event_ty as $crate::Event>::LIFETIMES != 0{
                    panic!("invalid type alias");
                }
            };
            $crate::register_hook_raw::<$event_ty>(val);
        }
    };
}
