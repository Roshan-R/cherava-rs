use crate::models::workflow::Workflow;
use crate::repository::database::Database;
use mail_send::mail_builder::MessageBuilder;
use mail_send::SmtpClientBuilder;

pub async fn send_workflow_updated_mail(workflow: &Workflow, db: Database) {
    // Assuming the user already has an associated mail with it
    let user = db.get_user_from_user_id(workflow.user_id).unwrap();

    let name = user.name.unwrap();
    let email = user.email.unwrap();

    let message = MessageBuilder::new()
        .from(("Cherava", "cherava-bot@outlook.com"))
        .to(vec![(name.as_str(), email.as_str())])
        .subject("Your Workflow has been changed")
        .html_body("<h1>Hello, world!</h1>")
        .text_body("Hello world!");

    SmtpClientBuilder::new("smtp-mail.outlook.com", 587)
        .implicit_tls(false)
        .credentials((
            std::env::var("SMTP_USERNAME").unwrap().as_str(),
            std::env::var("SMTP_PASSWORD").unwrap().as_str(),
        ))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
