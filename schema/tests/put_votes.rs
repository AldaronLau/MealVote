use schema::put::votes::Meal;
use validation::meal_id::MealId;

#[test]
fn meal() {
    let meal = Meal {
        meal_id: MealId::new(1).unwrap(),
        vote: true,
    };
    let muon = muon_rs::to_string(&meal).unwrap();

    insta::assert_snapshot!(muon);
}
