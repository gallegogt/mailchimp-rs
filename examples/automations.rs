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

use mailchimp::types::RecipientType;
use mailchimp::MailchimpApi;
use mailchimp::{Automations, AutomationsFilter};
use std::collections::HashMap;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);

    // Create instance of Automations
    let automations = Automations::new(api);
    let mut last_automation_id = String::from("");

    // Iterate through the existing Automations
    for w in automations.iter(AutomationsFilter::default()) {
        let settings = w.settings.as_ref().unwrap();
        last_automation_id = w.id.as_ref().unwrap().to_string();
        println!("Automation");
        println!("ID                {:?}", w.id);
        println!("Title             {:?}", settings.title);
        println!("Emails Sent       {:?}", w.emails_sent);
        println!("Report Summary    {:?}", w.report_summary);
        println!("Start Time        {:?}", w.start_time);
        println!("Create Time       {:?}", w.create_time);
        println!("Status            {:?}", w.status);
        println!("Tracking          {:?}", w.tracking);
        println!("Trigger Settings  {:?}", w.trigger_settings);
        println!("Recipients        {:?}", w.recipients);
        println!("=============================================")
    }

    // Example that show you, how to get specific automations by ID
    println!("\n\n");
    let workflow = automations
        .get_automation_workflow_info(last_automation_id.as_str(), HashMap::new())
        .unwrap();
    println!(
        "Configuracion del workflow ID:{}  \n{:?}",
        last_automation_id, workflow.settings
    );

    // Example of Filtered by status
    println!("\n\n");
    let mut filter = HashMap::new();
    filter.insert("status".to_string(), "sending".to_string());

    for w in automations.iter(AutomationsFilter {
        status: Some("sending".to_string()),
        fields: None,
        exclude_fields: None,
        count: Some(50),
        offset: Some(0),
        before_send_time: None,
        since_send_time: None,
        before_create_time: None,
        since_create_time: None,
    }) {
        let settings = w.settings.as_ref().unwrap();
        println!("Automatizacion");
        println!("ID                {:?}", w.id);
        println!("Title             {:?}", settings.title);
        println!("Emails Sent       {:?}", w.emails_sent);
        println!("Report Summary    {:?}", w.report_summary);
        println!("Start Time        {:?}", w.start_time);
        println!("Create Time       {:?}", w.create_time);
        println!("Status            {:?}", w.status);
        println!("Tracking          {:?}", w.tracking);
        println!("Trigger Settings  {:?}", w.trigger_settings);
        println!("Recipients        {:?}", w.recipients);
        println!("=============================================")
    }

    // ==================== Workflow Emails ========== ====
    // Example of get Workflow Emails
    let emails_resp = workflow.get_workflow_emails();

    match emails_resp {
        Ok(workflow_emails) => {
            for e in workflow_emails {
                println!("\nWorkflow Emails");
                println!("ID             {:?}", e.id);
                println!("Emails Sent    {:?}", e.emails_sent.as_ref().unwrap());
                println!("Start Time     {:?}", e.start_time.as_ref().unwrap());
                println!("Create time    {:?}", e.create_time.as_ref().unwrap());
                println!(
                    "Recipients        {:?}",
                    e.recipients.as_ref().unwrap_or(&RecipientType::default())
                );
                println!("Report Summary           {:?}", e.report_summary.as_ref().unwrap());
            }
        }
        Err(e) => println!("{:?}", e),
    }

    // ============= Workflow Emails Get Info =======================
    let we_info = workflow.get_automation_workflow_email_info("000da526de");

    match we_info {
        Ok(we) => {
            println!("\nWorkflow Emails");
            println!("ID                {:?}", we.id.unwrap_or("".to_string()));
            println!(
                "Emails Sent            {:?}",
                we.emails_sent.as_ref().unwrap_or(&0)
            );
            println!("Start Time      {:?}", we.start_time.as_ref().unwrap());
            println!("Create Time {:?}", we.create_time.as_ref().unwrap());
            println!("Recipients        {:?}", we.recipients.as_ref().unwrap());
            println!(
                "Report Summary           {:?}",
                we.report_summary.as_ref().unwrap()
            );
        }
        Err(e) => println!("Workflow Email Info Error {:?}", e),
    }
}
