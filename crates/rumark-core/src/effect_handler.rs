use std::future::Future;

use crate::{AnyEffect, Effect, ElementContext};

pub trait AnyEffectHandler<C: ElementContext> {
    type Future: Future<Output = ()>;
    fn handle<'a>(&self, context: C, effect: AnyEffect<'a>) -> Result<Self::Future, AnyEffect<'a>>;
}

impl<C: ElementContext, E: Effect, Fut: Future<Output = E::Return>, F: Fn(C, E) -> Fut>
    EffectHandler<C, E> for F
{
    type Future = Fut;

    fn handle(&self, context: C, effect: E) -> Self::Future {
        (self)(context, effect)
    }
}

pub trait EffectHandler<C: ElementContext, E: Effect> {
    type Future: Future<Output = E::Return>;
    fn handle(&self, context: C, effect: E) -> Self::Future;
}
