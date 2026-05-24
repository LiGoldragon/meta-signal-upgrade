//! Signal contract scaffold for the owner-only `upgrade` surface.
//!
//! U1 intentionally carries no domain operations. U3 populates this
//! channel with catalogue policy and selector authority operations from
//! the predecessor owner contracts.

use nota_codec::{NotaEnum, NotaRecord};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
use signal_sema::SemaObservation;

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum UnimplementedReason {
    NotBuiltYet,
    IntegrationNotLanded,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RequestUnimplemented {
    pub reason: UnimplementedReason,
}

signal_channel! {
    channel OwnerUpgrade {
    }
    reply Reply {
        RequestUnimplemented(RequestUnimplemented),
    }
    observable {
        filter default;
        operation_event OperationReceived;
        effect_event EffectEmitted;
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OperationReceived {
    pub operation: OperationKind,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct EffectEmitted {
    pub observation: SemaObservation,
}
