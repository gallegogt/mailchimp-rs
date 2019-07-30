///
/// Dependencies:
///
/// # This library is meant to be used on development or testing environments
/// # in which setting environment variables is not practical.
/// dotenv = "^0.13"
///
/// Requirements:
///
/// To run this example you need to create a archive named ``.env`` in the root of the directory with the following info
/// MAILCHIMP_API_KEY=<API KEY>
///
use dotenv::dotenv;
use std::env;

use mailchimp::{ListFilter, Lists, MailchimpApi};
use std::collections::HashMap;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);

    // Create Instance of Lists
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
