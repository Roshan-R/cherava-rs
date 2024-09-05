use crate::{
    controllers::{email::send::send_workflow_updated_mail, scraper::scrape},
    repository::database::Database,
};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn schedule_workflows(
    sched: &JobScheduler,
    db: Database,
) -> Result<(), JobSchedulerError> {
    let workflows = db.get_all_workflows();
    for workflow in workflows {
        let db = db.clone();
        let job = Job::new_async(workflow.cron.clone().as_str(), move |_uuid, _lock| {
            let mut w = db.get_workflow_by_workflow_id(workflow.clone().id).unwrap();
            Box::pin({
                let db = db.clone();
                async move {
                    let data = scrape(w.url.to_owned(), w.selector.to_owned())
                        .await
                        .unwrap();
                    if data != w.data {
                        w.data = data;
                        db.update_workflow(&w).unwrap();
                        send_workflow_updated_mail(&w, db).await;
                    }
                }
            })
        })?;
        sched.add(job).await.unwrap();
    }
    Ok(())
}
