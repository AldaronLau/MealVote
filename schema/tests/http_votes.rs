use schema::http::votes::PutRequest;

#[test]
fn put_request() {
    let put = PutRequest { has_vote: true };
    let muon = muon_rs::to_string(&put).unwrap();

    insta::assert_snapshot!(muon);
}
