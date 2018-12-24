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

    // Get information about all lists in the account.
    let lists_c = client.get_lists(HashMap::new());
    let mut list_id = String::new();

    match lists_c {
        Ok(lists) => {
            for w in &lists {
                list_id = w.id().unwrap().clone();
                println!("\n\nList");
                println!("\tid    {:?}", w.id());
                println!("\tName    {:?}", w.name());
                println!("\tStats   {:?}", w.stats());
                println!("=============================================");
            }
        }
        Err(e) => println!("{:?}", e),
    };

    // Get information about a specific list in your Mailchimp account.
    // Results include list members who have signed up but haven’t confirmed
    // their subscription yet and unsubscribed or cleaned.
    let r_list = client.get_list_info(list_id.as_str(), HashMap::new());
    match r_list {
        Ok(list) => {
            println!("\n\nList");
            println!("\tid    {:?}", list.id());
            println!("\tName    {:?}", list.name());
            println!("\tStats   {:?}", list.stats());
            println!("=============================================")
        }
        Err(e) => println!("{:?}", e),
    };
}
