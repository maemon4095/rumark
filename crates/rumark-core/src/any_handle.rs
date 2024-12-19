use crate::{AnyEffect, AnyEffectHandler, Effect, EffectHandler, ElementContext};
use std::marker::PhantomData;

pub struct AnyHandle<C: ElementContext, E: Effect, H: EffectHandler<C, E>>(H, PhantomData<(E, C)>);

impl<C: ElementContext, E: Effect, H: EffectHandler<C, E>> AnyHandle<C, E, H> {
    pub fn new(h: H) -> Self {
        Self(h, PhantomData)
    }
}

impl<C: ElementContext, E: Effect, H: EffectHandler<C, E>> AnyEffectHandler<C>
    for AnyHandle<C, E, H>
{
    fn handle<'a>(&self, context: C, e: AnyEffect<'a>) -> Result<(), AnyEffect<'a>> {
        let (slot, e) = e.try_cast::<E>()?;
        let result = self.0.handle(context, e);
        slot.set(result);
        Ok(())
    }
}
