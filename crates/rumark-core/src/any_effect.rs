use crate::Effect;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

pub struct AnyEffect<'a> {
    slot: &'a mut dyn Any,
}

pub struct EffectSlot<E: Effect> {
    state: EffectSlotState<E>,
}

enum EffectSlotState<E: Effect> {
    Unused(E),
    Handled(Arc<Mutex<Option<E::Return>>>),
}

pub struct EffectReturnSlot<E: Effect> {
    slot: Arc<Mutex<Option<E::Return>>>,
}

impl<'a> AnyEffect<'a> {
    pub fn new<E: Effect>(slot: &'a mut EffectSlot<E>) -> Self {
        Self {
            slot: slot as &mut dyn Any,
        }
    }

    pub fn try_cast<E: Effect>(self) -> Result<(EffectReturnSlot<E>, E), Self> {
        if !self.slot.is::<EffectSlot<E>>() {
            return Err(self);
        }

        let slot: &mut EffectSlot<E> = self.slot.downcast_mut().unwrap();
        let state = &mut slot.state;

        let arc = Arc::new(Mutex::new(None));
        match std::mem::replace(state, EffectSlotState::Handled(Arc::clone(&arc))) {
            EffectSlotState::Handled(_) => {
                panic!("tried to cast already handled effect.")
            }
            EffectSlotState::Unused(v) => {
                let return_slot = EffectReturnSlot { slot: arc };
                Ok((return_slot, v))
            }
        }
    }
}

impl<E: Effect> EffectSlot<E> {
    pub fn new(effect: E) -> Self {
        Self {
            state: EffectSlotState::Unused(effect),
        }
    }

    pub fn take(self) -> Result<E::Return, Self> {
        match self.state {
            state @ EffectSlotState::Unused(_) => Err(Self { state }),
            EffectSlotState::Handled(arc) => {
                let mut lock = arc.lock().unwrap();
                match lock.take() {
                    Some(v) => Ok(v),
                    None => {
                        drop(lock);
                        Err(Self {
                            state: EffectSlotState::Handled(arc),
                        })
                    }
                }
            }
        }
    }
}

impl<E: Effect> EffectReturnSlot<E> {
    pub fn set(self, result: E::Return) {
        let mut lock = self.slot.lock().unwrap();
        *lock = Some(result);
    }
}
