use std::ptr::{self, NonNull};

use anyhow::Result;

use crate::Event;

struct Opaque(());

pub(crate) struct ErasedHook {
    data: NonNull<Opaque>,
    call: unsafe fn(NonNull<Opaque>, NonNull<Opaque>, NonNull<Opaque>),
}

impl ErasedHook {
    pub(crate) fn new_dynamic<H: Fn() -> Result<()> + 'static + Send + Sync>(
        hook: H,
    ) -> ErasedHook {
        fn call<F: Fn() -> Result<()> + 'static + Send + Sync>(
            hook: NonNull<Opaque>,
            _event: NonNull<Opaque>,
            result: NonNull<Opaque>,
        ) {
            let hook: NonNull<F> = hook.cast();
            let result: NonNull<Result<()>> = result.cast();
            let hook: &F = unsafe { hook.as_ref() };
            let res = hook();
            unsafe { ptr::write(result.as_ptr(), res) }
        }

        unsafe {
            ErasedHook {
                data: NonNull::new_unchecked(Box::into_raw(Box::new(hook)) as *mut Opaque),
                call: call::<H>,
            }
        }
    }

    pub(crate) fn new<E: Event, F: Fn(&mut E) -> Result<()>>(hook: F) -> ErasedHook {
        fn call<E: Event, F: Fn(&mut E) -> Result<()>>(
            hook: NonNull<Opaque>,
            event: NonNull<Opaque>,
            result: NonNull<Opaque>,
        ) {
            let hook: NonNull<F> = hook.cast();
            let mut event: NonNull<E> = event.cast();
            let result: NonNull<Result<()>> = result.cast();
            let hook: &F = unsafe { hook.as_ref() };
            let res = unsafe { hook(event.as_mut()) };
            unsafe { ptr::write(result.as_ptr(), res) }
        }

        unsafe {
            ErasedHook {
                data: NonNull::new_unchecked(Box::into_raw(Box::new(hook)) as *mut Opaque),
                call: call::<E, F>,
            }
        }
    }

    pub(crate) unsafe fn call<E: Event>(&self, event: &mut E) -> Result<()> {
        let mut res = Ok(());

        unsafe {
            (self.call)(
                self.data,
                NonNull::from(event).cast(),
                NonNull::from(&mut res).cast(),
            );
        }
        res
    }
}

unsafe impl Sync for ErasedHook {}
unsafe impl Send for ErasedHook {}
