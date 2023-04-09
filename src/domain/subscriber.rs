use crate::domain::{SubscriberEmail, SubscriberName};
use crate::routes::SubscriberCreateRequest;

#[derive(Debug)]
pub struct Subscriber {
    pub name: SubscriberName,
    pub email: SubscriberEmail,
}

impl TryFrom<SubscriberCreateRequest> for Subscriber {
    type Error = String;

    fn try_from(value: SubscriberCreateRequest) -> Result<Self, Self::Error> {
        Ok(Subscriber {
            name: SubscriberName::parse(value.name)?,
            email: SubscriberEmail::parse(value.email)?,
        })
    }
}
