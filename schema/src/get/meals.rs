use validation::{meal_id::MealId, meal_name::MealName};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ListMeal {
    pub id: MealId,
    pub name: MealName,
}

/// GET `/api/meals/list`
///
/// Get a list of available meals.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct List {
    pub meal: Vec<ListMeal>,
}
