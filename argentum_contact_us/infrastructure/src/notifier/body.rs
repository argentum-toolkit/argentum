use askama_escape::{Html, escape};

use argentum_contact_us_business::entity::Message;
use argentum_standard_business::data_type::id::Id;
use argentum_user_business::entity::user::Member;

fn sender_name<I>(message: &Message<I>, sender: Option<&Member<I>>) -> Option<String>
where
    I: Id,
{
    match sender {
        Some(member) => Some(format!(
            "{} {}",
            member.name.first,
            member
                .name
                .last
                .as_ref()
                .map(|last| last.to_string())
                .unwrap_or_default()
        )),
        None => message.name.clone(),
    }
}

pub fn build_subject<I>(
    message: &Message<I>,
    sender: Option<&Member<I>>,
    product_name: &str,
) -> String
where
    I: Id,
{
    let name = sender_name(message, sender).unwrap_or_else(|| "Guest".to_string());

    format!("{name} sent Contact US message. {product_name}")
}

pub fn build_body<I>(message: &Message<I>, sender: Option<&Member<I>>, product_name: &str) -> String
where
    I: Id,
{
    let subject = build_subject(message, sender, product_name);

    let (name, email) = match sender {
        Some(member) => (
            Some(format!(
                "{} {}",
                member.name.first,
                member
                    .name
                    .last
                    .as_ref()
                    .map(|last| last.to_string())
                    .unwrap_or_default()
            )),
            Some(member.email.as_string()),
        ),
        None => (
            message.name.clone(),
            message.email.as_ref().map(|e| e.as_string()),
        ),
    };

    let name = name.unwrap_or_default();
    let email = email.unwrap_or_default();

    let text = escape(&message.message, Html)
        .to_string()
        .replace('\n', "<br/>");

    format!(
        "<h2>{}</h2>\
        <p><b>Name:</b> {}</p>\
        <p><b>Email:</b> {}</p>\
        <p>{}</p>",
        escape(&subject, Html),
        escape(&name, Html),
        escape(&email, Html),
        text,
    )
}
