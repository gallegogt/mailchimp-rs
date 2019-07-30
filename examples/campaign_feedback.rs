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
use dotenv::dotenv;
use std::env;

use mailchimp::MailchimpApi;
use mailchimp::{CampaignFilter, Campaigns};

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);

    // Create Campaigns Instance
    let r_campaigns = Campaigns::new(api);
    let mut count = 0;
    // Get all campaigns in an account.
    for w in r_campaigns.iter(CampaignFilter::default()) {
        count += 1;
        println!("\n Campaign   {:}", count);
        println!(
            "\t Campaign Title  {:?}",
            w.settings.as_ref().unwrap().title
        );
        println!("=============================================");

        for feedback in w.get_feedbacks(None, None) {
            println!("Feedback: ");
            println!("Feedback ID: {:?}", &feedback.feedback_id);
            println!("Feedback Message: {:?}", &feedback.message);
            println!("Feedback Create By: {:?}", &feedback.created_by);
            println!("Feedback Source: {:?}", &feedback.source);

            println!("=============================================");
        }
    }
}
