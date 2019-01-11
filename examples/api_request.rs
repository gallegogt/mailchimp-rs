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
use std::env;

use mailchimp::types::AuthorizedAppsType;
use mailchimp::MailchimpApi;
use std::collections::HashMap;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_API_KEY"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API Instance
    let api = MailchimpApi::new(&apk);
    // Make the request to the endpoint /authorized-apps en Mailchimp
    let data = api.get::<AuthorizedAppsType>("authorized-apps", HashMap::new());

    match data {
        Ok(resp) => {
            // Iterate through response
            for app in resp.apps.iter() {
                println!("{:?}", app)
            }
        }
        Err(e) => println!("Error Title: {:?} \ndetail {:?}", e.title, e.detail),
    }
}
