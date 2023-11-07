use crate::models::{filtered_user::FilteredUser, user::User};

pub async fn filter_user(user: &User) -> FilteredUser {
    FilteredUser {
        id: user.id.to_string(),
        email: user.email.to_owned(),
        user_type: user.user_type.to_owned(),
        user_group: user.user_group.to_owned(),
        user_profile: user.user_profile.to_owned(),
    }
}
