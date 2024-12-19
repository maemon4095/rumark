use crate::Effect;
use std::any::Any;

pub struct AnyEffect<'a> {
    slot: &'a mut dyn Any,
}

pub struct EffectSlot<E: Effect> {
    state: EffectSlotState<E>,
}

enum EffectSlotState<E: Effect> {
    Unused(E),
    Handling,
    Handled(E::Return),
}

pub struct EffectReturnSlot<'a, E: Effect> {
    state: &'a mut EffectSlotState<E>,
}

impl<'a> AnyEffect<'a> {
    pub fn new<E: Effect>(slot: &'a mut EffectSlot<E>) -> Self {
        Self {
            slot: slot as &mut dyn Any,
        }
    }

    pub fn try_cast<E: Effect>(self) -> Result<(EffectReturnSlot<'a, E>, E), Self> {
        if !self.slot.is::<EffectSlot<E>>() {
            return Err(self);
        }

        let slot: &mut EffectSlot<E> = self.slot.downcast_mut().unwrap();
        let state = &mut slot.state;

        match std::mem::replace(state, EffectSlotState::Handling) {
            EffectSlotState::Handling | EffectSlotState::Handled(_) => {
                panic!("tried to cast already handled effect.")
            }
            EffectSlotState::Unused(v) => {
                let return_slot = EffectReturnSlot { state };
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
            state @ (EffectSlotState::Unused(_) | EffectSlotState::Handling) => Err(Self { state }),
            EffectSlotState::Handled(v) => Ok(v),
        }
    }
}

impl<'a, E: Effect> EffectReturnSlot<'a, E> {
    pub fn set(self, result: E::Return) {
        *self.state = EffectSlotState::Handled(result);
    }
}
