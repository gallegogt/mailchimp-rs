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
    let mut lists_c = lists.iter(ListFilter::default());
    let lists_it = lists_c.next();

    if let Some(l) = lists_it {
        println!("\n\nList");
        println!("\tName    {:?}", l.name.as_ref().unwrap());
        println!("=============================================");

        let mut m_iter = l.get_members(None);
        if let Some(c) = m_iter.next() {
            println!("\nMember:");
            println!("\tEmail Address       {:?}", c.email_address);
            println!("\tUnique Email Id     {:?}", c.unique_email_id);
            println!("\tEmail Type          {:?}", c.email_type);
            println!("\tStatus              {:?}", c.status);
            println!("\tUnsubscribe Reason  {:?}", c.unsubscribe_reason);
            println!("\tMerge Fields        {:?}", c.merge_fields);
            println!("\tInterests           {:?}", c.interests);
            println!("\tStats               {:?}", c.stats);
            println!("\tIP signup           {:?}", c.ip_signup);
            println!("\tTimestamp signup    {:?}", c.timestamp_signup);
            println!("\tIP Opt              {:?}", c.ip_opt);
            println!("\tTimestamp Opt       {:?}", c.timestamp_opt);
            println!("\tMember Rating       {:?}", c.member_rating);
            println!("\tLast Change         {:?}", c.last_changed);
            println!("\tVIP                 {:?}", c.vip);
            println!("\tEmail Client        {:?}", c.email_client);
            println!("\tLocation            {:?}", c.location);
            println!("\tMarketing permission {:?}", c.marketing_permissions);
            println!("\tLast Note           {:?}", c.last_note);
            println!("\tTags Count          {:?}", c.tags_count);
            println!("\tTags                {:?}", c.tags);
            println!("\tList ID             {:?}", c.list_id);
            println!("\tLinks               {:?}", c._links);
        }
    }
}
