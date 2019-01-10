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
        println!("\tid    {:?}", w.id.as_ref().unwrap().to_string());
        println!("\tName    {:?}", w.name.as_ref().unwrap());
        println!("\tStats   {:?}", w.stats.as_ref().unwrap());
        println!("=============================================");

        // Show last list activity symmary (today)
        let mut iter_act = w.get_activity(None, None);
        if let Some(activity) = iter_act.next() {
        // for activity in w.get_activity(None, None) {
            println!("\nList Activity");
            println!("Day {:?}", activity.day);
            println!("Emails Sent {:?}", activity.emails_sent);
            println!("Unique Opens {:?}", activity.unique_opens);
            println!("Recipient Clicks {:?}", activity.recipient_clicks);
            println!("Hard Bounce {:?}", activity.hard_bounce);
            println!("Soft Bounce {:?}", activity.soft_bounce);
            println!("Subs {:?}", activity.subs);
            println!("Other Adds {:?}", activity.other_adds);
            println!("Other removes {:?}", activity.other_removes);
        }
    }
}
