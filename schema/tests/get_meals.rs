use schema::get::meals::{Muon, Meal};
use validation::{meal_id::MealId, meal_name::MealName};

#[test]
fn list() {
    let meal_list = Muon {
        meal: Vec::from([
            Meal {
                id: MealId::new(1).unwrap(),
                name: MealName::new("Changas".to_string()).unwrap(),
            },
            Meal {
                id: MealId::new(2).unwrap(),
                name: MealName::new("Veggie Pasta".to_string()).unwrap(),
            },
        ]),
    };
    let muon = muon_rs::to_string(&meal_list).unwrap();
    eprintln!("{muon}");

    insta::assert_snapshot!(muon);
}
