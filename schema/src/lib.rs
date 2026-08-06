//! Internet communication and database storage format specification

/// MuON database files
pub mod file {
    /// File format for `/{home_id}/meals.muon`
    pub mod home_meals;
    /// File format for `/{home_id}/perms.muon`
    pub mod home_perms;
    /// File format for `/{home_id}/roles.muon`
    pub mod home_roles;
    /// File format for `/{home_id}/votes.muon`
    pub mod home_votes;
    /// File format for `/homes.muon`
    pub mod homes;
    /// File format for `/users.muon`
    pub mod users;
}

/// HTTP at `:mealvote_server/api/{mod_name}`
pub mod http {
    /// HTTP API for `/meals`
    pub mod meals;
    /// HTTP API for `/users`
    pub mod users;
    /// HTTP API for `/votes`
    pub mod votes;
}
