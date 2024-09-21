pub trait AuthUser {
    fn get_id(&self) -> String;
    fn get_username(&self) -> String;
    fn is_admin(&self) -> bool;
}
