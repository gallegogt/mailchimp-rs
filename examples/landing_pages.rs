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
use mailchimp::{LandingPages, MailchimpApi};

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

    // Create Conversations instance
    let conv = LandingPages::new(api);

    for it in conv.get_pages(None) {
        println!("\n\nPage: ");
        println!("\tid    {:?}", it.id);
        println!("\tname    {:?}", it.name);
        println!("\ttitle    {:?}", it.title);
        println!("\tdescription    {:?}", it.description);
        println!("\ttemplate_id    {:?}", it.template_id);
        println!("\tstatus    {:?}", it.status);
        println!("\tlist_id    {:?}", it.list_id);
        println!("\tstore_id    {:?}", it.store_id);
        println!("\turl    {:?}", it.url);
        println!("\tcreated_at    {:?}", it.created_at);
        println!("\tpublished_at    {:?}", it.published_at);
        println!("\tunpublished_at    {:?}", it.unpublished_at);
        println!("\tupdated_at    {:?}", it.updated_at);

        println!("=============================================");
    }

}
