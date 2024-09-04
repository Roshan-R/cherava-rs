use crate::{controllers::scraper::scrape, repository::database::Database};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub async fn schedule_workflows(
    sched: &JobScheduler,
    db: Database,
) -> Result<(), JobSchedulerError> {
    let workflows = db.get_all_workflows();
    for workflow in workflows {
        let db = db.clone();
        let job = Job::new_async(workflow.cron.clone().as_str(), move |_uuid, _lock| {
            let mut w = workflow.clone();
            Box::pin({
                let db = db.clone();
                async move {
                    let data = scrape(w.url.to_owned(), w.selector.to_owned())
                        .await
                        .unwrap();
                    if data != w.data {
                        w.data = data;
                        db.update_workflow(w).unwrap();
                    }
                }
            })
        })?;
        sched.add(job).await.unwrap();
    }
    Ok(())
}
