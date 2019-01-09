extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::Automations;
use mailchimp::{MailchimpApi, AutomationsFilter};

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&apk);

    // Ejemplo de como obtener todas la automatizaciones
    let automations = Automations::new(api);
    for workflow in automations.iter(AutomationsFilter::default()) {
        println!(
            "\nAutomation Workflow ID: {:?}  \n Title: {:?}",
            workflow.id,
            workflow.settings.as_ref().unwrap().title
        );

        for usr in workflow.get_subscribers_removed() {
            println!("Susbscriber Removed ");
            println!("ID: {:?}", &usr.id);
            println!("Email: {:?}", &usr.email_address);
        }
    }
}
