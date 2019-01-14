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

extern crate dotenv;
extern crate mailchimp;
extern crate md5;

use dotenv::dotenv;
use mailchimp::{ListFilter, Lists, MailchimpApi};
use std::env;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);

    // Create Lists instance
    let lists = Lists::new(api);

    // Get information about all lists in the account.
    let mut lists_c = lists.iter(ListFilter::default());
    let lists_it = lists_c.next();

    if let Some(l) = lists_it {
        println!("\n\nList");
        println!("\tName    {:?}", l.name.as_ref().unwrap());
        println!("=============================================");

        // Get list members
        let mut m_iter = l.get_members(None);
        let mut email_address = "".to_string();

        // select one element of the list
        if let Some(c) = m_iter.next()  {
            email_address = c.email_address.clone();
            println!("\nMember:");
            println!("\tEmail Address       {:?}", email_address);
            println!("\tUnique Email Id     {:?}", c.unique_email_id);
            println!("\tEmail Type          {:?}", c.email_type);
        }

        let digest = md5::compute(email_address);
        let digest_str = format!("{:x}", digest);

        // Show all activity for one member list
        for ma in l.get_member_activity(&digest_str) {
            println!("\nActivity for {:?}: ", &digest_str);
            println!("\nAction: {:?}", ma.action);
            println!("\nTimestamp: {:?}", ma.timestamp);
            println!("\nUrl: {:?}", ma.url);
            println!("\nActivity Type: {:?}", ma.activity_type);
            println!("\nCampaign ID: {:?}", ma.campaign_id);
            println!("\nTitle: {:?}", ma.title);
            println!("\nParent Campaign: {:?}", ma.parent_campaign);
        }
    }
}