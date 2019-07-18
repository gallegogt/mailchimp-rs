# [UNOFFICIAL] Mailchimp API Implementation


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

### More examples in ``examples/*``

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

### Example get all mailchimp reports

```rust
  extern crate mailchimp;

  use mailchimp::types::ReportType;
  use mailchimp::{MailchimpApi, Reports};

  fn main() {
      // Init API
      let api = MailchimpApi::new("<API_KEY>");

      // Create Instance of Reports
      let reports = Reports::new(api);

      // Get information about all reports.
      let reports_iter = reports.iter_reports(None);

      for report in reports_iter {
          print_report_type(&report);
          println!("=============================================");
      }
  }

  fn print_report_type(report: &ReportType) {
      println!("\n\nReports");
      println!("\tid    {:?}", report.id);
      println!("\tcampaign_title    {:?}", report.campaign_title);
      println!("\treport_type    {:?}", report.report_type);
      println!("\tlist_id    {:?}", report.list_id);
      println!("\tlist_is_active    {:?}", report.list_is_active);
      println!("\tlist_is_active    {:?}", report.list_is_active);
      println!("\tlist_name    {:?}", report.list_name);
      println!("\tsubject_line    {:?}", report.subject_line);
      println!("\tpreview_text    {:?}", report.preview_text);
      println!("\temails_sent    {:?}", report.emails_sent);
      println!("\tabuse_reports    {:?}", report.abuse_reports);
      println!("\tunsubscribed    {:?}", report.unsubscribed);
      println!("\tsend_time    {:?}", report.send_time);
      println!("\trss_last_send    {:?}", report.rss_last_send);
      println!("\tbounces    {:?}", report.bounces);
      println!("\tforwards    {:?}", report.forwards);
      println!("\topens    {:?}", report.opens);
      println!("\tclicks    {:?}", report.clicks);
      println!("\topens    {:?}", report.opens);
      println!("\tfacebook_likes    {:?}", report.facebook_likes);
      println!("\tlist_stats    {:?}", report.list_stats);
      println!("\tab_split    {:?}", report.ab_split);
      println!("\ttimewarp    {:?}", report.timewarp);
      println!("\ttimeseries    {:?}", report.timeseries);
      println!("\tshare_report    {:?}", report.share_report);
      println!("\tecommerce    {:?}", report.ecommerce);
      println!("\tdelivery_status    {:?}", report.delivery_status);
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
    * ✅ **Webhooks**
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

### ✅ Ping
### ✅ Conversations
  * ✅ Get a list of conversations
  * ✅ Get information about a conversation
  * ✅ **Messages**
    * ✅ Post a new conversation message
    * ✅ Get conversation messages
    * ✅ Get a specific conversation message

### 🔘 Reports
    * ✅ Get campaign reports
    * ✅ Get a specific campaign report
    * 🔘 Campaign Abuse
    * 🔘 Campaign Advice
    * 🔘 Campaign Open Reports
    * 🔘 Click Reports
    * 🔘 Get domain performance stats
    * 🔘 Ecommerce Product Activity
    * 🔘 EepURL Reports
    * 🔘 Email Activity
    * 🔘 Google Analytics
    * 🔘 Location
    * 🔘 Sent To
    * 🔘 Sub-Reports
    * 🔘 Unsubscribes

### 🔘 E-commerce Stores
### 🔘 Reporting
### 🔘 Campaign Folders
### 🔘 Batch Operations
### 🔘 Connected Sites
### 🔘 Batch Webhooks
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


# Contributing

You want to contribute to this project? Wow, thanks! So please just fork it and send me a pull request.

# Support My Efforts

I programmed this lib for fun and do my best effort to support those that have issues with it, please return the favor and support me.

[![paypal](https://www.paypalobjects.com/en_US/i/btn/btn_donateCC_LG.gif)](https://www.paypal.me/reiloygt)
