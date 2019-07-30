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
///
use dotenv::dotenv;
use std::env;

use mailchimp::{ApiRoot, MailchimpApi};
use std::collections::HashMap;

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to obtain the Mailchimp Credential
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_API_KEY"));
    let apk = env_mailchimp.next().unwrap().1;

    // Init API Instance
    let api = MailchimpApi::new(&apk);
    // Create a instance of API Root, to get the info about the current account
    let api_root = ApiRoot::new(api);

    // Get the info about the current account
    let info = api_root.get_info(HashMap::new());

    match info {
        Ok(account) => {
            println!("Account Info");
            println!("Account Name      {:?}", account.account_name);
            println!("Email             {:?}", account.email);
            println!("First Name        {:?}", account.first_name);
            println!("Role              {:?}", account.role);
            println!("Member Since      {:?}", account.member_since);
            println!("Account Timezone  {:?}", account.account_timezone);
            println!("Account Industry  {:?}", account.account_industry);
            println!("Total Subscribers {:?}", account.total_subscribers);
            println!("Industry Stats    {:?}", account.industry_stats);
            println!("=============================================")
        }
        Err(e) => println!("{:?}", e),
    };
}
