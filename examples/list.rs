extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::{ListFilter, Lists, MailchimpApi};
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
    let lists = Lists::new(api);
    // Get information about all lists in the account.
    let lists_c = lists.iter(ListFilter::default());
    let mut list_id = String::new();
    let mut count = 0;

    for w in lists_c {
        list_id = w.id.unwrap().clone();
        count += 1;
        println!("\n\nList {:?}", count);
        println!("\tid    {:?}", list_id);
        println!("\tName    {:?}", w.name);
        println!("\tStats   {:?}", w.stats);
        println!("=============================================");
    }

    // Get information about a specific list in your Mailchimp account.
    // Results include list members who have signed up but haven’t confirmed
    // their subscription yet and unsubscribed or cleaned.
    let r_list = lists.get_list_info(list_id.as_str(), HashMap::new());
    match r_list {
        Ok(list) => {
            println!("\n\nList");
            println!("\tid    {:?}", list.id);
            println!("\tName    {:?}", list.name);
            println!("\tStats   {:?}", list.stats);
            println!("=============================================")
        }
        Err(e) => println!("{:?}", e),
    };
}
