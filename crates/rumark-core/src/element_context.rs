use crate::Effect;

pub trait ElementContext {
    fn perform<E: Effect>(&self, effect: E) -> E::Return;
}
