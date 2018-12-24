extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::MailchimpClient;
use std::collections::HashMap;

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let dc = env_mailchimp.next().unwrap().1;
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let client = MailchimpClient::new(&dc, &apk);

    // Get all campaigns in an account.
    let r_campaigns = client.get_campaigns(HashMap::new());
    let mut campaign_id = String::new();

    match r_campaigns {
        Ok(campaigns) => {
            println!("Totals: {:?}", campaigns.len());
            for w in &campaigns {
                campaign_id = w.id().unwrap().clone();
                println!("\n Campaign");
                println!("\t Campaign Type    {:?}", w.campaign_type());
                println!("\t Campaign Title   {:?}", w.settings().unwrap().title);
                println!("\t Emails Sent   {:?}", w.emails_sent().unwrap());
                println!("\t Report Summary   {:?}", w.report_summary().unwrap());
                println!("\t Delivery Status   {:?}", w.delivery_status().unwrap());
                println!("=============================================");
            }
        }
        Err(e) => println!("{:?}", e),
    };

    // Get information about a specific campaign.
    let r_camp = client.get_campaign_info(campaign_id.as_str(), HashMap::new());
    match r_camp {
        Ok(list) => {
            println!("\nCampaign");
            println!("\tid    {:?}", list.id());
            println!("\tCampaign Title   {:?}", list.settings().unwrap().title);
        }
        Err(e) => println!("{:?}", e),
    };
}
