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
use mailchimp::{Conversations, MailchimpApi};

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
    let conv = Conversations::new(api);
    let mut conversation_id = "".to_string();

    for it in conv.get_conversations(None) {
        conversation_id = it.id.clone();
        println!("\n\nConversation: ");
        println!("\tid    {:?}", it.id);
        println!("\tMessage Count       {:?}", it.message_count);
        println!("\tUnread Messages     {:?}", it.unread_messages);
        println!("\tFrom Label          {:?}", it.from_label);
        println!("\tFrom Email          {:?}", it.from_email);
        println!("\tSubject             {:?}", it.subject);
        println!("\tLast Message        {:?}", it.last_message);
        println!("=============================================");
    }

    match conv.get_conversation(&conversation_id) {
        Ok(cv) => print!("\nConversation Info:\n{:?}", cv),
        Err(e) => print!("Get conversation Info Error {:?}", e),
    };
}
