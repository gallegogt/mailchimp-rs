extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;
use mailchimp::{MailchimpApi, AuthorizedApps, AuthorizedFilter};

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
    let authorized_apps = AuthorizedApps::new(api);
    // Ejemplo de como obtener todas la automatizaciones
    let apps = authorized_apps.iter(AuthorizedFilter::default());
    let mut count = 0;

    for app in apps {
        count += 1;
        println!("\nApp {:?}", count);
        println!("ID   {:?}", app.id());
        println!("Name   {:?}", app.name());
        println!("Descriptions   {:?}", app.description());
        println!("Users   {:?}", app.users());
        println!("===========================");
    }
}
