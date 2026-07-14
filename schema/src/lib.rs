//! Internet communication and database storage format specification

/// MuON database files
pub mod file {
    /// File format for `meals.muon`
    pub mod meals;
    /// File format for `users.muon`
    pub mod users;
    /// File format for `votes.muon`
    pub mod votes;
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
