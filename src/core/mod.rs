use user::User;
use uuid::Uuid;

pub mod user;
pub struct Core {
    users: Vec<User>,
}

impl Core {
    pub fn new() -> Self {
        Core { users: vec![] }
    }

    pub fn get_user<'a>(&'a self, id: Uuid) -> Option<&'a User> {
        
        //for user in self.users.iter() {
        //    if id.eq(&user.id) {
        //        return Some(user);
        //    }
        //}
        return None;
    }
}
