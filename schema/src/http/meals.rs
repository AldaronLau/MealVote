use validation::{meal_id::MealId, meal_name::MealName};

/// List for [`GetResponse`] request.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetResponseMeal {
    pub id: MealId,
    pub name: MealName,
}

/// GET `/api/meals`
///
/// Get a list of available meals.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct GetResponse {
    pub meal: Vec<GetResponseMeal>,
}
