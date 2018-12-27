extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::{MailchimpApi, Automations};
use mailchimp::{RecipientType, AutomationCampaignSettingsType, AutomationTriggerType};

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let dc = env_mailchimp.next().unwrap().1;
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&dc, &apk);
    let automations = Automations::new(api);
    let automat_resp = automations.create_automation(
        RecipientType::create("<list_id>", "<scope_id>"),
        AutomationTriggerType::create("<workflow_type>"),
        Some(AutomationCampaignSettingsType::create("<from_name>", "<reply_to>"))
    );

    match automat_resp {
        Ok(value) => println!("{:?}", value),
        Err(e) => println!("{:?}", e)
    }
}
