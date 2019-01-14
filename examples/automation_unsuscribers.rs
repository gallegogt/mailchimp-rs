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

use mailchimp::Automations;
use mailchimp::{AutomationsFilter, MailchimpApi};

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);

    // Create the Instance Automations
    let automations = Automations::new(api);

    // Iterate through the existing Automations
    for workflow in automations.iter(AutomationsFilter::default()) {
        println!(
            "\nAutomation Workflow ID: {:?}  \n Title: {:?}",
            workflow.id,
            workflow.settings.as_ref().unwrap().title
        );
        // Example that how to obtail susbscribers removed
        for usr in workflow.get_subscribers_removed() {
            println!("Susbscriber Removed ");
            println!("ID: {:?}", &usr.id);
            println!("Email: {:?}", &usr.email_address);
        }
    }
}
