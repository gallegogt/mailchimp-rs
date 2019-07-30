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

        // select one element of the list
        if let Some(c) = m_iter.next() {
            println!("\nMember:");
            println!("\tEmail Address       {:?}", c.email_address);
            println!("\tUnique Email Id     {:?}", c.unique_email_id);
            println!("\tEmail Type          {:?}", c.email_type);

            // Show all notes
            for ma in c.get_notes(None) {
                println!("\nNote for        {:?}: ", c.id);
                println!("\nContent Note:   {:?}", ma.note);
                println!("\nCrate By:       {:?}", ma.created_by);
                println!("\nUpdated At:     {:?}", ma.updated_at);
                println!("\nCreate At:      {:?}", ma.created_at);
            }
        }
    }
}
