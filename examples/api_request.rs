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
use mailchimp::MailchimpApi;
use std::env;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_API_KEY"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API Instance
    let api = MailchimpApi::new(&apk);
    // Ping
    let ping_rs = api.ping();
    match ping_rs {
        Ok(value) => {
            println!("Ping ... {:?}", value);
        }
        Err(e) => println!("Error Title: {:?} \ndetail {:?}", e.title, e.detail),
    }
}
