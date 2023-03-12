use reqwest::Client;
use std::sync::Once;

static mut HTTP_CLIENT: Option<Client> = None;
static INIT: Once = Once::new();

pub fn provide_client() -> Client {
    // NOTE  Keep HTTP_CLIENT a singleton shared across API clients
    unsafe {
        INIT.call_once(|| {
            if HTTP_CLIENT.is_none() {
                HTTP_CLIENT = Some(Client::new());
            }
        });
        HTTP_CLIENT.clone().unwrap()
    }
}
