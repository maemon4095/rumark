use crate::{AnyEffect, AnyEffectHandler, Effect, EffectHandler, EffectReturnSlot, ElementContext};
use std::{future::Future, marker::PhantomData, pin::Pin, task::Poll};

pub struct AnyHandle<C: ElementContext, E: Effect, H: EffectHandler<C, E>>(H, PhantomData<(E, C)>);

impl<C: ElementContext, E: Effect, H: EffectHandler<C, E>> AnyHandle<C, E, H> {
    pub fn new(h: H) -> Self {
        Self(h, PhantomData)
    }
}

impl<C: ElementContext, E: Effect, H: EffectHandler<C, E>> AnyEffectHandler<C>
    for AnyHandle<C, E, H>
{
    type Future = Handle<C, E, H>;

    fn handle<'a>(&self, context: C, e: AnyEffect<'a>) -> Result<Self::Future, AnyEffect<'a>> {
        let (slot, e) = e.try_cast::<E>()?;

        Ok(Handle {
            state: HandleState::Polling {
                slot,
                future: self.0.handle(context, e),
            },
        })
    }
}
pub struct Handle<C: ElementContext, E: Effect, H: EffectHandler<C, E>> {
    state: HandleState<C, E, H>,
}

enum HandleState<C: ElementContext, E: Effect, H: EffectHandler<C, E>> {
    Polling {
        slot: EffectReturnSlot<E>,
        future: H::Future,
    },
    Done,
}

impl<C: ElementContext, E: Effect, H: EffectHandler<C, E>> Future for Handle<C, E, H> {
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        unsafe {
            let ptr = &mut self.get_unchecked_mut().state;
            match ptr {
                HandleState::Polling { future, .. } => {
                    let mut future = Pin::new_unchecked(future);

                    let Poll::Ready(v) = future.as_mut().poll(cx) else {
                        return Poll::Pending;
                    };

                    drop(future);
                    let HandleState::Polling { slot, .. } =
                        std::mem::replace(ptr, HandleState::Done)
                    else {
                        unreachable!()
                    };
                    slot.set(v);
                    Poll::Ready(())
                }
                HandleState::Done => panic!("poll was called on already done future."),
            }
        }
    }
}
