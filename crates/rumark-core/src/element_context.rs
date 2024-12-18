use std::future::Future;

use crate::Effect;

pub trait ElementContext {
    type Perform<E: Effect>: Future<Output = E::Return>;

    fn perform<E: Effect>(&self, effect: E) -> Self::Perform<E>;
}
