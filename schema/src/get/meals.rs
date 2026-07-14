use validation::{meal_id::MealId, meal_name::MealName};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Meal {
    pub id: MealId,
    pub name: MealName,
}

/// GET `/api/meals`
///
/// Get a list of available meals.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Muon {
    pub meal: Vec<Meal>,
}
