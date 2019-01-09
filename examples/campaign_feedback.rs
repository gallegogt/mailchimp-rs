extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::MailchimpApi;
use mailchimp::{CampaignFilter, Campaigns};

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&apk);

    // Get all campaigns in an account.
    let r_campaigns = Campaigns::new(api);
    let mut count = 0;
    for w in r_campaigns.iter(CampaignFilter::default()) {
        count += 1;
        println!("\n Campaign {:}", count);
        println!("\t Campaign Title   {:?}", w.settings.as_ref().unwrap().title);
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
