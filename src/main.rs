#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use anyhow::{anyhow, Result};

use chrono::{Date, Duration, Utc};

use std::env::var;
mod on_call;
mod slack;

use crate::on_call::{OnCall, OnCallResponse};
use crate::slack::SlackIncomingWebhookMessage;

fn main() -> Result<()> {
    let pd_api_token = var("PD_API_TOKEN")?;
    let slack_webhook_url = var("SLACK_WEBHOOK_URL")?;
    let schedule_ids = var("SCHEDULE_IDS")?;
    let schedule_ids: Vec<&str> = schedule_ids.split(',').collect();

    let mut pd_req = ureq::get("https://api.pagerduty.com/oncalls");
    pd_req.query("time_zone", "UTC");
    for schedule_id in schedule_ids {
        pd_req.query("schedule_ids[]", schedule_id);
    }

    let today: Date<Utc> = Utc::now().date();
    let tomorrow: Date<Utc> = today + Duration::days(1);

    pd_req.query("since", &today.format("%Y-%m-%d").to_string());
    // The range is not inclusive at the end so this must be 1 day further on than the target date
    pd_req.query(
        "until",
        &(tomorrow + Duration::days(1))
            .format("%Y-%m-%d")
            .to_string(),
    );
    pd_req.query("earliest", "false");
    pd_req.set("Authorization", &format!("Token token={}", pd_api_token));

    // TODO: LOG DEBUG THIS pd_reqUEST println!("{:?}", pd_req);

    let response = pd_req.call();
    if !response.ok() {
        return Err(anyhow!("Error contacting PD API"));
    }

    let on_call_response = &response.into_string()?;
    // TODO: LOG DEBUG THIS RESPONSE
    let on_call_response: OnCallResponse = serde_json::from_str(on_call_response)?;
    // TODO: LOG DEBUG THIS STRUCT
    // println!("{:?}", on_call_response);

    let today_summary = get_on_call_summary(&on_call_response, today);
    let tomorrow_summary = get_on_call_summary(&on_call_response, tomorrow);
    let stitched_summary = format!(
        "*Today's On Call*: {}\n\n*Tomorrow's On Call*: {}",
        today_summary, tomorrow_summary
    );

    let slack_message = SlackIncomingWebhookMessage::new(stitched_summary);
    let mut slack_req = ureq::post(&slack_webhook_url);
    slack_req.set("Content-type", "application/json");
    let slack_response = slack_req.send_string(&serde_json::to_string(&slack_message)?);

    if !slack_response.ok() {
        return Err(anyhow!("Error contacting Slack webhook"));
    }

    Ok(())
}

fn get_on_call_summary(on_call_response: &OnCallResponse, comparison_date: Date<Utc>) -> String {
    let dates_on_call: Vec<&OnCall> = on_call_response
        .on_calls
        .iter()
        .filter(|x| x.start.date() == comparison_date)
        .collect();

    let mut on_call_summary = String::with_capacity(30);

    for on_call in &dates_on_call {
        on_call_summary.push_str(&format!(
            "\n{}: Level {}",
            on_call.user.name, on_call.escalation_level
        ))
    }

    on_call_summary
}
