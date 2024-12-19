use crate::{AnyEffect, Effect, ElementContext};

pub trait AnyEffectHandler<C: ElementContext> {
    fn handle<'a>(&self, context: C, effect: AnyEffect<'a>) -> Result<(), AnyEffect<'a>>;
}

impl<C: ElementContext, E: Effect, F: Fn(C, E) -> E::Return> EffectHandler<C, E> for F {
    fn handle(&self, context: C, effect: E) -> E::Return {
        (self)(context, effect)
    }
}

pub trait EffectHandler<C: ElementContext, E: Effect> {
    fn handle(&self, context: C, effect: E) -> E::Return;
}
