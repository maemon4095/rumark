use crate::{any_handle::AnyHandle, AnyEffectHandler, Effect, EffectHandler, ElementContext};

pub trait Element: Sized {
    type Context: ElementContext;
    fn handle_any_with<H: AnyEffectHandler<Self::Context>>(self, handler: H) -> Self;
}

pub trait ElementExt: Element {
    fn handle_with<E: Effect, H: EffectHandler<Self::Context, E>>(self, handler: H) -> Self {
        self.handle_any_with(AnyHandle::new(handler))
    }

    fn handle<E: Effect, F: Fn(Self::Context, E) -> E::Return>(self, handler: F) -> Self {
        self.handle_with(handler)
    }
}
impl<E: Element> ElementExt for E {}
