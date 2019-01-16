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
    for l in lists.iter(ListFilter::default()) {
        println!("\n\nList");
        println!("\tName    {:?}", l.name.as_ref().unwrap());
        println!("=============================================");

        for c in l.get_signup_forms(None) {
            println!("\nInterest Category:");
            println!("\tHeader           {:?}", c.header);
            println!("\tContent          {:?}", c.contents);
            println!("\tStyle            {:?}", c.styles);
            println!("\tSignup Form Url  {:?}", c.signup_form_url);
            println!("\tList ID          {:?}", c.list_id);
        }
    }
}
