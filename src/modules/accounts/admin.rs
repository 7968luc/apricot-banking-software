use super::*;

pub trait Admin {
    /// deletes the user from this username.
    fn delete_user<T: ToString>(username: T);
    /// reverses the last transaction made.
    fn reverse_transaction<T: ToString>(username: T);
}
