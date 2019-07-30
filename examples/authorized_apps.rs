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
use mailchimp::{AuthorizedApps, AuthorizedFilter, MailchimpApi};
use std::env;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);
    // Create instance of AuthorizedApps
    let authorized_apps = AuthorizedApps::new(api);
    // Iterate through the existing Authorized Apps
    let apps = authorized_apps.iter(AuthorizedFilter::default());
    let mut count = 0;

    for app in apps {
        count += 1;
        println!("\nApp {:?}", count);
        println!("ID   {:?}", app.id);
        println!("Name   {:?}", app.name);
        println!("Descriptions   {:?}", app.description);
        println!("Users   {:?}", app.users);
        println!("===========================");
    }
}
