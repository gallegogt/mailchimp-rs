extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::types::AuthorizedAppsType;
use mailchimp::MailchimpApi;
use std::collections::HashMap;

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&apk);
    // Se realiza una petición al endpoint /authorized-apps
    let data = api.get::<AuthorizedAppsType>("authorized-apps", HashMap::new());

    match data {
        Ok(resp) => {
            // Se recorren todas las aplicaciones que tienen acceso al Mailchimp
            for app in resp.apps.iter() {
                println!("{:?}", app)
            }
        }
        Err(e) => println!("Error Title: {:?} \ndetail {:?}", e.title, e.detail),
    }
}
