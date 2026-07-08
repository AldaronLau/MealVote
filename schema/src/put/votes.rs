use validation::meal_id::MealId;

/// PUT `/api/votes/meal`
///
/// Set the status of whether or not the user has voted for a meal.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Meal {
    pub meal_id: MealId,
    pub vote: bool,
}
