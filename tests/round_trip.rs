use nota_codec::{Decoder, Encoder, NotaDecode, NotaEncode};
use owner_signal_upgrade::{
    ObserverFilter, ObserverSubscriptionToken, Operation, OperationKind, Reply,
    RequestUnimplemented, UnimplementedReason,
};
use signal_frame::{
    ExchangeFrame, ExchangeFrameBody, ExchangeIdentifier, ExchangeLane, LaneSequence,
    RequestPayload, SessionEpoch, SubscriptionTokenInner,
};

const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn encode<T: NotaEncode>(value: &T) -> String {
    let mut encoder = Encoder::new();
    value.encode(&mut encoder).expect("encode nota");
    encoder.into_string()
}

fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = encode(&value);
    assert_eq!(encoded, expected);

    let mut decoder = Decoder::new(&encoded);
    let recovered = T::decode(&mut decoder).expect("decode nota");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn scaffold_exposes_only_observer_operations_until_owner_surface_lands() {
    let token = ObserverSubscriptionToken::new(SubscriptionTokenInner::new(11));

    assert_eq!(
        Operation::Tap(ObserverFilter::All).kind(),
        OperationKind::Tap
    );
    assert_eq!(Operation::Untap(token).kind(), OperationKind::Untap);
}

#[test]
fn request_unimplemented_round_trips_through_nota() {
    round_trip_nota(
        Reply::RequestUnimplemented(RequestUnimplemented {
            reason: UnimplementedReason::NotBuiltYet,
        }),
        "(RequestUnimplemented (NotBuiltYet))",
    );
}

#[test]
fn observer_operation_survives_frame_round_trip() {
    let frame = ExchangeFrame::<Operation, Reply>::new(ExchangeFrameBody::Request {
        exchange: exchange(),
        request: Operation::Tap(ObserverFilter::All).into_request(),
    });
    let bytes = frame.encode().expect("encode frame");
    let decoded = ExchangeFrame::<Operation, Reply>::decode(&bytes).expect("decode frame");

    assert_eq!(decoded, frame);
}
