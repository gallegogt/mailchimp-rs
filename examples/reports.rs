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

use mailchimp::types::ReportType;
use mailchimp::{MailchimpApi, Reports};

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Init API
    let api = MailchimpApi::new(&apk);

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
