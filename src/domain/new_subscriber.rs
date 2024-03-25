use super::{subscriber_name::SubscriberName, SubscriberEmail};

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: SubscriberEmail,
    pub name: SubscriberName,
}
