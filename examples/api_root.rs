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

    // Ejemplo de como obtener todas la automatizaciones
    let info = client.get_account_info(HashMap::new());

    match info {
        Ok(account) => {
            println!("Información de la cuenta");
            println!("Nombre de la Cuenta   {:?}", account.account_name);
            println!("Email                 {:?}", account.email);
            println!("Nombre                {:?}", account.first_name);
            println!("Role                  {:?}", account.role);
            println!("Fecha de creaciónn    {:?}", account.member_since);
            println!("Timezone              {:?}", account.account_timezone);
            println!("Ruburo                {:?}", account.account_industry);
            println!("Total de subcriptores {:?}", account.total_subscribers);
            println!("Estadisticas          {:?}", account.industry_stats);
            println!("=============================================")
        }
        Err(e) => println!("{:?}", e),
    };
}
