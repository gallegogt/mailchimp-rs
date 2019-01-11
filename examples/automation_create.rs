///
/// Dependencies:
///
/// # This library is meant to be used on development or testing environments
/// # in which setting environment variables is not practical.
/// dotenv = "^0.13"
///
/// Requirements:
///
/// To run this example you need to create a archive named ``.env`` in the root of the directory with the following info
/// MAILCHIMP_API_KEY=<API KEY>
///

extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::types::{AutomationCampaignSettingsType, AutomationTriggerType, RecipientType};
use mailchimp::{Automations, MailchimpApi};

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);
    // Create Automation Instance
    let automations = Automations::new(api);

    // Example of how to create new automations
    let automat_resp = automations.create_automation(
        RecipientType::create("<list_id>", "<scope_id>"),
        AutomationTriggerType::create("<workflow_type>"),
        Some(AutomationCampaignSettingsType::create(
            "<from_name>",
            "<reply_to>",
        )),
    );

    match automat_resp {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("{:?}", e),
    }
}
