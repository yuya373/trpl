#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        // compile error
        // resolve client module under tests module
        // reference to tests::client module
        // client::connect();

        // resolve client module under root (chapter7) module
        ::client::connect();
        // using super to move up one module in hierarchy from current module.
        // chapter7 (super) -> tests   (current)
        //                  -> client
        super::client::connect();

        use super::client;
        client::connect();

        use super::*;
        client::connect();
        network::connect();
        network::server::connect();
    }
}

pub mod client;
pub mod network;

mod outermost {
    pub fn middle_function() {}
    fn middle_secret_function() {}

    pub fn try_me() {
        middle_secret_function();
        inside::inner_function();
        // private fn
        // inside::secret_function();
        inside::try_me();
    }

    mod inside {
        pub fn inner_function() {}
        fn secret_function() {}

        pub fn try_me() {
            inner_function();
            secret_function();
        }
    }
}

pub fn try_me() {
    outermost::middle_function();
    // private fn
    // outermost::middle_secret_function();
    // private mod
    // outermost::inside::inner_function();
    // private mod
    // outermost::inside::secret_function();

    outermost::try_me();
}
