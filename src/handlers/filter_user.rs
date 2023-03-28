use crate::models::response::FilteredUser;
use crate::models::users::User;

pub fn filter_user_record(user: &User) -> FilteredUser {
    return FilteredUser {
        id: user.id.to_string(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        role: user.role.to_owned(),
        photo: user.photo.to_owned(),
        verified: user.verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        deleted_at: user.deleted_at,
    };
}
