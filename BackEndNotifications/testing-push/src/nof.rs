pub mod nof {
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct NotificationData {
        pub title: String,
        pub body: String,
        pub icon: String,
        pub badge: String,
        pub link: String,
    }
}
