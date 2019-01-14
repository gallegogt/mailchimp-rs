///
/// Example used to share statistics from a specific Mailchimp account through MQTT and be monitored by Grafana
///
/// Requirements:
///     You need to have some MQTT Broker active, to send the information through it
///
/// Create archive named ``.env`` in the root of the directory with the following info and run the example
/// MAILCHIMP_API_KEY=<API KEY>
/// MQTT_USER=<MQTT_USER>
/// MQTT_PASSWORD=<MQTT_PASSWORD>
/// MQTT_HOST=<MQTT_HOST>
///
/// Dependencies
///
/// # This library is meant to be used on development or testing environments
/// # in which setting environment variables is not practical.
/// dotenv = "^0.13"
/// # Eclipse Paho MQTT Rust Client Library
/// paho-mqtt = {version="^0.5", default-features=false}
///
extern crate dotenv;
extern crate mailchimp;
extern crate paho_mqtt as mqtt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use std::process;

use mailchimp::types::{
    ApiRootType, AutomationWorkflowType, CampaignType, ListType, StatisticsType,
};
use mailchimp::{
    ApiRoot, Automations, AutomationsFilter, CampaignFilter, Campaigns, ListFilter, Lists,
    MailchimpApi,
};

use std::collections::HashMap;
use std::time::Duration;

///
/// ====================================================================
/// Mailchimp user account Stats
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpAccountStats {
    // Name of measurement
    #[serde(default)]
    pub measurement_name: String,
    // Mailchimp client or account name
    #[serde(default)]
    pub client_name: String,
    // ============ STATS =============
    /// The total number of subscribers across all lists in the account.
    #[serde(default)]
    pub total_subscribers: u64,
    /// The average unique open rate for all campaigns in the account’s specified industry.
    #[serde(default)]
    pub industry_stats_open_rate: f32,
    /// The average bounce rate for all campaigns in the account’s specified industry.
    #[serde(default)]
    pub industry_stats_bounce_rate: f32,
    /// The average unique click rate for all campaigns in the account’s specified industry.
    #[serde(default)]
    pub industry_stats_click_rate: f32,
}

impl MailchimpAccountStats {
    pub fn create_stats(ar: &ApiRootType) -> Self {
        MailchimpAccountStats {
            measurement_name: "mailchimp_account".to_string(),
            client_name: ar.account_name.clone(),
            total_subscribers: ar.total_subscribers,
            industry_stats_open_rate: ar.industry_stats.open_rate,
            industry_stats_bounce_rate: ar.industry_stats.bounce_rate,
            industry_stats_click_rate: ar.industry_stats.click_rate,
        }
    }
}

///
/// ====================================================================
///
/// Automatizaciones
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpAutomationStats {
    // Name of measurement
    #[serde(default)]
    pub measurement_name: String,
    // Mailchimp client or account name
    #[serde(default)]
    pub client_name: String,

    // ============ STATS =============
    ///The current status of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The total number of emails sent for the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,
    // Mailchimp client or account name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients_list_name: Option<String>,
    /// The title of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The total number of opens for a campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary_opens: Option<u64>,
    /// The number of unique opens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary_unique_opens: Option<u64>,
    /// The number of unique opens divided by the total number of successful deliveries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary_open_rate: Option<f32>,
    /// The total number of clicks for an campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary_clicks: Option<u64>,
    /// The number of unique clicks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary_subscriber_clicks: Option<u64>,
    /// The number of unique clicks, divided by the total number of successful deliveries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub report_summary_click_rate: Option<f32>,
}

impl MailchimpAutomationStats {
    pub fn create_stats<'a>(data: &AutomationWorkflowType, account_name: &'a str) -> Self {
        let r_list_name = if let Some(r) = &data.recipients {
            r.list_name.clone()
        } else {
            None
        };

        let s_title = if let Some(r) = &data.settings {
            r.title.clone()
        } else {
            None
        };

        let mut rs_v = (0 as u64, 0 as u64, 0.0, 0 as u64, 0 as u64, 0.0);

        if let Some(rp) = &data.report_summary {
            rs_v.0 = rp.opens;
            rs_v.1 = rp.unique_opens;
            rs_v.2 = rp.open_rate;
            rs_v.3 = rp.clicks;
            rs_v.4 = rp.subscriber_clicks;
            rs_v.5 = rp.click_rate;
        }

        MailchimpAutomationStats {
            measurement_name: "mailchimp_automation".to_string(),
            client_name: account_name.to_string(),
            status: data.status.clone(),
            emails_sent: data.emails_sent.clone(),
            recipients_list_name: r_list_name,
            title: s_title,
            report_summary_opens: Some(rs_v.0),
            report_summary_unique_opens: Some(rs_v.1),
            report_summary_open_rate: Some(rs_v.2),
            report_summary_clicks: Some(rs_v.3),
            report_summary_subscriber_clicks: Some(rs_v.4),
            report_summary_click_rate: Some(rs_v.5),
        }
    }
}

///
/// ====================================================================
///
/// Campaigns
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpCampaignStats {
    // Name of measurement
    #[serde(default)]
    pub measurement_name: String,
    // Mailchimp client or account name
    #[serde(default)]
    pub client_name: String,

    // ============ STATS =============
    /// There are four types of campaigns you can create in Mailchimp. A/B Split
    /// campaigns have been deprecated and variate campaigns should be used instead.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub campaign_type: Option<String>,
    /// The current status of the campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The total number of emails sent for this campaign.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<u64>,
    /// List Name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients_list_name: Option<String>,
    /// Count of the recipients on the associated list. Formatted as an integer..
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients_recipient_count: Option<u64>,
    /// The title of the Automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The total number of opens for a campaign.
    #[serde(default)]
    pub report_summary_opens: u64,
    /// The number of unique opens.
    #[serde(default)]
    pub report_summary_unique_opens: u64,
    /// The number of unique opens divided by the total number of successful deliveries.
    #[serde(default)]
    pub report_summary_open_rate: f32,
    /// The total number of clicks for an campaign.
    #[serde(default)]
    pub report_summary_clicks: u64,
    /// The number of unique clicks.
    #[serde(default)]
    pub report_summary_subscriber_clicks: u64,
    /// The number of unique clicks, divided by the total number of successful deliveries.
    #[serde(default)]
    pub report_summary_click_rate: f32,
    /// The total orders for a campaign.
    #[serde(default)]
    pub report_summary_ecommerce_total_orders: u64,
    /// The total spent for a campaign. Calculated as the sum of
    /// all order totals with no deductions.
    #[serde(default)]
    pub report_summary_ecommerce_total_spent: f32,
    /// The total revenue for a campaign. Calculated as the sum of
    /// all order totals minus shipping and tax totals.
    #[serde(default)]
    pub report_summary_ecommerce_total_revenue: f32,
}

impl MailchimpCampaignStats {
    pub fn create_stats<'a>(data: &CampaignType, account_name: &'a str) -> Self {
        let mut settings = (Some(String::new()), None, Some(String::new()));
        if let Some(rc) = data.recipients.as_ref() {
            settings.0 = rc.list_name.clone();
            settings.1 = rc.recipient_count;
        }
        if let Some(ss) = data.settings.as_ref() {
            settings.2 = ss.title.clone();
        }

        let mut report_summary = (0, 0, 0.0, 0, 0, 0.0, 0, 0.0, 0.0);
        if let Some(rs) = data.report_summary.as_ref() {
            report_summary.0 = rs.opens;
            report_summary.1 = rs.unique_opens;
            report_summary.2 = rs.open_rate;
            report_summary.3 = rs.clicks;
            report_summary.4 = rs.subscriber_clicks;
            report_summary.5 = rs.click_rate;

            if let Some(e) = &rs.ecommerce {
                report_summary.6 = e.total_orders;
                report_summary.7 = e.total_spent;
                report_summary.8 = e.total_revenue;
            }
        }

        MailchimpCampaignStats {
            measurement_name: "mailchimp_campaigns".to_string(),
            client_name: account_name.to_string(),
            campaign_type: data.campaign_type.clone(),
            status: data.status.clone(),
            emails_sent: data.emails_sent,
            recipients_list_name: settings.0,
            recipients_recipient_count: settings.1,
            title: settings.2,
            report_summary_opens: report_summary.0,
            report_summary_unique_opens: report_summary.1,
            report_summary_open_rate: report_summary.2,
            report_summary_clicks: report_summary.3,
            report_summary_subscriber_clicks: report_summary.4,
            report_summary_click_rate: report_summary.5,
            report_summary_ecommerce_total_orders: report_summary.6,
            report_summary_ecommerce_total_spent: report_summary.7,
            report_summary_ecommerce_total_revenue: report_summary.8,
        }
    }
}

///
/// ====================================================================
///
/// Lists
///
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailchimpListStats {
    // Name of measurement
    #[serde(default)]
    pub measurement_name: String,
    // Mailchimp client or account name
    #[serde(default)]
    pub client_name: String,

    // ============ STATS =============
    /// The name of the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Stats for the list. Many of these are cached for at least five minutes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<StatisticsType>,
}

impl MailchimpListStats {
    pub fn create_stats<'a>(data: &ListType, account_name: &'a str) -> Self {
        MailchimpListStats {
            measurement_name: "mailchimp_lists".to_string(),
            client_name: account_name.to_string(),
            name: data.name.clone(),
            stats: data.stats.clone(),
        }
    }
}

///
/// ====================================================================
///
///
fn connect_mqtt<'a>(host: &'a str, user_name: &'a str, password: &'a str) -> mqtt::Client {
    let endpoint = String::from("tcp://") + host;
    let create_options = mqtt::CreateOptionsBuilder::new()
        .server_uri(endpoint)
        .persistence(mqtt::PersistenceType::None)
        .finalize();
    let cli = mqtt::Client::new(create_options).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .user_name(user_name)
        .password(password)
        .clean_session(true)
        .finalize();

    // Connect and wait for it to complete or fail
    match cli.connect(conn_opts) {
        Ok(_) => println!("Connect: Successs \n"),
        Err(e) => {
            println!("Unable to connect:\n\t{:?}", e);
            process::exit(1);
        }
    };

    cli
}

fn send_message<'a, T>(cli: &mqtt::Client, topic: &'a str, mas: &T)
where
    T: serde::Serialize,
{
    let payload = serde_json::to_string(mas);
    println!("Topic: {:?} payload {:?}", topic, payload);
    // Create a message and publish it
    let msg = mqtt::MessageBuilder::new()
        .topic(topic)
        .payload(payload.unwrap().as_str())
        .qos(1)
        .finalize();

    if let Err(e) = cli.publish(msg) {
        println!("Error sending message: {:?}", e);
    }
}

fn disconnect(cli: &mqtt::Client) {
    // Disconnect from the broker
    cli.disconnect(None).unwrap();
}

fn main() {
    // Init dotenv
    dotenv().ok();
    // Filter the env vars to get the Mailchimp Credential
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    let mut mqtt_settings = (String::new(), String::new(), String::new());

    for v in env::vars() {
        if v.0 == "MQTT_USER" {
            mqtt_settings.0 = v.1.clone();
        }
        if v.0 == "MQTT_PASSWORD" {
            mqtt_settings.1 = v.1.clone();
        }
        if v.0 == "MQTT_HOST" {
            mqtt_settings.2 = v.1.clone();
        }
    }

    println!("MQTT Settings: {:?}", mqtt_settings);

    let mqtt_client = connect_mqtt(
        mqtt_settings.2.as_str(),
        mqtt_settings.0.as_str(),
        mqtt_settings.1.as_str(),
    );

    // Init API
    let api = MailchimpApi::new(&apk);
    // ========== Mailchimp Account Stats ========
    let api_root = ApiRoot::new(api.clone());

    // Ejemplo de como obtener todas la automatizaciones
    let account_info = api_root.get_info(HashMap::new());
    let mut account_name = String::new();

    match account_info {
        Ok(account) => {
            let stats = MailchimpAccountStats::create_stats(&account);
            account_name = account.account_name.clone();
            send_message(&mqtt_client, "mailchimp/stats/account", &stats);
        }
        Err(e) => println!("Error: {:?}", e),
    }

    // ========== Mailchimp Automations ========
    let automation = Automations::new(api.clone());

    for aut in automation.iter(AutomationsFilter::default()) {
        let stats = MailchimpAutomationStats::create_stats(&aut, &account_name);
        send_message(&mqtt_client, "mailchimp/stats/automations", &stats);
    }
    // ========== Mailchimp Campaigns ========
    let campaigns = Campaigns::new(api.clone());

    for data in campaigns.iter(CampaignFilter::default()) {
        let stats = MailchimpCampaignStats::create_stats(&data, &account_name);
        send_message(&mqtt_client, "mailchimp/stats/campaigns", &stats);
    }

    // ========== Mailchimp Lists ========
    let lists = Lists::new(api.clone());

    for data in lists.iter(ListFilter::default()) {
        let stats = MailchimpListStats::create_stats(&data, &account_name);
        send_message(&mqtt_client, "mailchimp/stats/lists", &stats);
    }

    disconnect(&mqtt_client);
}
