use schema::http::meals::{GetResponse, GetResponseMeal};
use validation::{meal_id::MealId, meal_name::MealName};

#[test]
fn get_response() {
    let get = GetResponse {
        meal: Vec::from([
            GetResponseMeal {
                id: MealId::new(1).unwrap(),
                name: MealName::new("Changas".to_string()).unwrap(),
            },
            GetResponseMeal {
                id: MealId::new(2).unwrap(),
                name: MealName::new("Veggie Pasta".to_string()).unwrap(),
            },
        ]),
    };
    let muon = muon_rs::to_string(&get).unwrap();
    eprintln!("{muon}");

    insta::assert_snapshot!(muon);
}
