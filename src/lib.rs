pub mod controller {
    pub mod v1_controller;
    pub mod v2_controller;

    #[cfg(test)]
    mod v1_controller_test;
    #[cfg(test)]
    mod v2_controller_test;
}

pub mod middleware {
    pub mod auth;
    pub mod context;
}
