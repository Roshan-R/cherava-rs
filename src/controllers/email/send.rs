use crate::controllers::database::Database;
use crate::models::workflow::Workflow;
use crate::CONFIG;

use log::info;
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
        .subject(format!(
            "Cherava: Content update on workflow - {}",
            workflow.id
        ))
        .text_body(format!(
            "Your workflow {} had the following update received {}",
            workflow.id, workflow.data
        ));

    info!(
        "Sending an email notification to {} due to change in workflow {}",
        email, workflow.id
    );

    SmtpClientBuilder::new(CONFIG.smtp_hostname.as_str(), 587)
        .implicit_tls(false)
        .credentials((CONFIG.smtp_username.as_str(), CONFIG.smtp_password.as_str()))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap();
}
