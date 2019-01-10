extern crate dotenv;
extern crate mailchimp;
use dotenv::dotenv;
use mailchimp::{ListFilter, Lists, MailchimpApi};
use std::env;

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&apk);
    let lists = Lists::new(api);
    // Get information about all lists in the account.
    let lists_c = lists.iter(ListFilter::default());
    let mut count = 0;

    for w in lists_c {
        count += 1;
        println!("\n\nList {:?}", count);
        println!("\tName    {:?}", w.name.as_ref().unwrap());
        println!("=============================================");

        for c in w.get_locations(None, None) {
            println!("\nLocations:");
            println!("\tCountry {:?}", c.country);
            println!("\tCountry Code {:?}", c.cc);
            println!("\tPercent {:?}", c.percent);
            println!("\tTotal {:?}", c.total);
        }
    }
}
