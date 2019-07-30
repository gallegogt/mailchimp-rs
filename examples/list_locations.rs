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
    let lists_c = lists.iter(ListFilter::default());
    let mut count = 0;

    for w in lists_c {
        count += 1;
        println!("\n\nList {:?}", count);
        println!("\tName    {:?}", w.name.as_ref().unwrap());
        println!("=============================================");

        for c in w.get_locations(None, None) {
            println!("\nLocations:");
            println!("\tCountry {:?}", c.country);
            println!("\tCountry Code {:?}", c.cc);
            println!("\tPercent {:?}", c.percent);
            println!("\tTotal {:?}", c.total);
        }
    }
}
