use super::*;

/* TODO:
*  We received numerous reports
*  that the software lacks any privilege
*  checking that allows users to create
*  accounts, as well as a credential leak.
*  We'll have to patch this ASAP.
*/

pub trait Admin {
    /// create a user
    fn create_user<T: ToString>(username: T, password: T);
    /// deletes the user from this username.
    fn delete_user<T: ToString>(username: T);
    /// reverses the last transaction made.
    fn reverse_transaction<T: ToString>(username: T);
}
