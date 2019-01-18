# Mailchimp API Implementation

### Library for the development of applications that require the use of the [Mailchimp](https://developer.mailchimp.com) API, using the Rust programming language

NOTA: NOTE: The library is in development, in the section "**Implementation Status**" I show you that I have implemented and that not.
---

### What can you do with the library?

  - [x] ✅ Get the general information about your Mailchimp Account
      * Total Subscribers
      * Industry Stats
      * Account Industry
      * ...
  - [x] ✅ Get all your Mailchimp Automations, and information of each of them
  - [x] ✅ Get all your Mailchimp Campaigns, and information of each of them
  - [x] ✅ Get all your Mailchimp Lists, and information of each of them
  - [x] ✅ And more...


### ✅ Example of how you can extract all the automations

Dependencies:

```toml
[dependencies]
mailchimp = "0.1"
```

Rust Code:

```rust
  extern crate mailchimp;
  use mailchimp::MailchimpApi;
  use mailchimp::{Automations, AutomationsFilter};
  use std::collections::HashMap;

  fn main() {
      // Init the API instance with the API KEY
      let api = MailchimpApi::new("<API_KEY>");

      // Create instance of Automations
      let automations = Automations::new(api);

      // Now you can go through all the automations and display information on
      // each of the automations.
      for w in automations.iter(AutomationsFilter::default()) {
          let settings = w.settings.as_ref().unwrap();
          last_automation_id = w.id.as_ref().unwrap().to_string();
          println!("Automation");
          println!("ID                {:?}", w.id);
          println!("Title             {:?}", settings.title);
          println!("Emails Sent       {:?}", w.emails_sent);
          println!("Report Summary    {:?}", w.report_summary);
          println!("Start Time        {:?}", w.start_time);
          println!("Create Time       {:?}", w.create_time);
          println!("Status            {:?}", w.status);
          println!("Tracking          {:?}", w.tracking);
          println!("Trigger Settings  {:?}", w.trigger_settings);
          println!("Recipients        {:?}", w.recipients);
          println!("=============================================")
      }
  }
```

### More examples in ``examples/*``

# Status of Development

Below I present a list where I will be updating to the extent possible the different endpoints supported by the library

### Authorized Apps
  * ✅ Link your application
  * ✅ Get a list of authorized apps
  * ✅ Get information about a specific authorized app

### Automations
  * ✅ Create a new Automation
  * ✅ Get a list of Automations
  * ✅ Get information about a specific Automation workflow
  * ✅ Update an Automation
  * ✅ [Action] Pause all emails in an Automation workflow
  * ✅ [Action] Start all emails in an Automation workflow

  * **Emails**
    * ✅ Get a list of automated emails in a workflow
    * ✅ Get information about a specific workflow email
    * ✅ Update an Automation workflow email
    * ✅ Delete a workflow email
    * ✅ Pause an automated email
    * ✅ Start an automated email

    * **Queue**
      * ✅ View queued subscribers for an automated email
      * ✅ View specific subscriber in email queue
      * ✅ Add a subscriber to a workflow email

  * **Removed Subscribers**
    * ✅ Remove subscriber from a workflow
    * ✅  View all subscribers removed from a workflow


### List
  * ✅ Create a new list
  * ✅ Batch sub/unsub list members
  * ✅ Get information about all lists
  * ✅ Get information about a specific list

    * ✅ **Abuse Reports**
    * ✅ **Activity**
    * ✅ **Clients**
    * ✅ **Growth History**
    * ✅ **Interest Categories**
    * ✅ **Locations**
    * ✅ **Members**
    * ✅ **Merge Fields**
    * ✅ **Segments**
    * ✅ **Signup Forms**
    * 🔘 **Webhooks**
    * ✅ **Locations**

### Campaigns
  * ✅ Create a new campaign
  * ✅ Get all campaigns
  * ✅ Get information about a specific campaign
  * ✅ Update the settings for a campaign
  * ✅ Delete a campaign
  * ✅ Cancel a campaign
  * ✅ Resend a campaign
  * ✅ Pause an RSS-Driven campaign
  * ✅ Replicate a campaign
  * ✅ Resume an RSS-Driven campaign
  * ✅ Schedule a campaign
  * ✅ Send a campaign
  * ✅ Send a test email
  * ✅ Unschedule a campaign

  * ✅ **Content**
  * ✅  **Feedback**
  * ✅  **Send Checklist**

### 🔘 Ping
### 🔘 Reporting
### 🔘 Reports

### 🔘 Campaign Folders
### 🔘 Batch Operations
### 🔘 Connected Sites
### 🔘 Conversations
### 🔘 Batch Webhooks
### 🔘 E-commerce Stores
### 🔘 Facebook Ads
### 🔘 File Manager Files
### 🔘 File Manager Folders
### 🔘 Google Ads
### 🔘 Landing Pages
### 🔘 Landing Pages
### 🔘 Search Campaigns
### 🔘 Search Members
### 🔘 Template Folders
### 🔘 Templates
