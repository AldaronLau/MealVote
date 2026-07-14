use schema::put::votes::Meal;

#[test]
fn meal() {
    let meal = Meal {
        has_vote: true,
    };
    let muon = muon_rs::to_string(&meal).unwrap();

    insta::assert_snapshot!(muon);
}
