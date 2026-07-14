/// PUT `/api/votes/{meal_id}`
///
/// Set the status of whether or not the user has voted for a meal.
#[derive(serde::Deserialize, serde::Serialize)]
pub struct PutRequest {
    pub has_vote: bool,
}
