use super::{CreateUser, UserStorage};

pub struct UserModel {}

impl UserStorage for UserModel {
    fn get_by_id(id: i64) -> Self {
        todo!()
    }

    fn get_by_email(email: String) -> Self {
        todo!()
    }

    fn create(user: &CreateUser) -> Self {
        todo!()
    }
}
