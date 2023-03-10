use yewdux::store::Store;

use crate::auth::Auth;

#[derive(Clone, Default, PartialEq, Eq, Store, Debug)]
pub struct AuthStore {
    pub auth: Auth,
}
