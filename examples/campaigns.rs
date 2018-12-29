extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::MailchimpApi;
use std::collections::HashMap;
use mailchimp::{Campaigns, CampaignFilter};

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
    let mut campaign_id = String::new();
    let r_campaigns = Campaigns::new(api);

    let mut count = 0;
    for w in r_campaigns.iter(CampaignFilter::default()) {
        count += 1;
        campaign_id = w.id().as_ref().unwrap().to_string();
        println!("\n Campaign {:}", count);
        println!("\t Campaign Type    {:?}", w.campaign_type());
        println!("\t Campaign Title   {:?}", w.settings().unwrap().title);
        println!("\t Emails Sent   {:?}", w.emails_sent().unwrap());
        if let Some(rs) = w.report_summary() {
            println!("\t Report Summary   {:?}", rs);
        }
        if let Some(rs) = w.delivery_status() {
            println!("\t Report Summary   {:?}", rs);
        }
        println!("=============================================");
    }

    // Get information about a specific campaign.
    let r_camp = r_campaigns.get_campaign_info(campaign_id.as_str(), HashMap::new());
    match r_camp {
        Ok(list) => {
            println!("\nCampaign");
            println!("\tid    {:?}", list.id());
            println!("\tCampaign Title   {:?}", list.settings().unwrap().title);
        }
        Err(e) => println!("{:?}", e),
    };
}
