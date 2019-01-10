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

        for ar in w.get_abuse_reports(None, None, None) {
            specific_abuse_report = ar.id;
            println!("\nAbuse Report:");
            println!("\tID: {:?}", specific_abuse_report);
            println!("\tEmail Address: {:?}", ar.email_address);
            println!("\tMerge Fields: {:?}", ar.merge_fields);
            println!("\tVip: {:?}", ar.vip);
            println!("\tDate: {:?}", ar.date);
        }
    }
}
