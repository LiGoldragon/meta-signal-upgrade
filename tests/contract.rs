use meta_signal_upgrade::{
    ByteViewable, Query, QueryRequest, Response, Restorable, Signal, Signalizable,
    UnimplementedReason,
};
#[test]
fn query_round_trips_as_fresh_signal() {
    let q = Query::Query(QueryRequest::All);
    let r = Signal::<Query>::from(q.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(r.restore().expect("restore"), q);
}
#[test]
fn rejection_round_trips_as_fresh_signal() {
    let v = Response::RequestUnimplemented(UnimplementedReason::NotBuiltYet);
    let r = Signal::<Response>::from(v.signalize().expect("signalize").bytes().to_vec());
    assert_eq!(r.restore().expect("restore"), v);
}
#[cfg(feature = "datom")]
#[test]
fn query_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};
    let q = Query::Query(QueryRequest::All);
    let text = q.clone().datomize(vec![]).protosize().textualize();
    let mut p = Potential::<Query>::from(text);
    assert_eq!(
        p.actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024
        })
        .expect("actualize"),
        q
    );
}
