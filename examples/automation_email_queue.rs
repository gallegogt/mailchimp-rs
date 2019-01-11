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

    // Create Automations Instance
    let automations = Automations::new(api);

    // Iterate through the existing Automations Workflows
    for workflow in automations.iter(AutomationsFilter::default()) {
        println!(
            "\nAutomation Workflow ID: {:?}  \n Title: {:?}",
            workflow.id,
            workflow.settings.as_ref().unwrap().title
        );

        let emails_resp = workflow.get_workflow_emails();

        match emails_resp {
            Ok(workflow_emails) => {
                for e in workflow_emails {
                    println!("\nWorkflow Emails");
                    println!("ID   {:?}", e.id);
                    println!("Emails Sents      {:?}", e.emails_sent.as_ref().unwrap());
                    println!("Start Time        {:?}", e.start_time.as_ref().unwrap());
                    println!("Create Time       {:?}", e.create_time.as_ref().unwrap());
                    println!("Report Summary    {:?}", e.report_summary.as_ref().unwrap());

                    // Example of how to get the existing Automations Email Queue
                    for aeq in e.get_email_queue() {
                        println!("\nAutomation Email Queue");
                        println!("{:?}", aeq);
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
