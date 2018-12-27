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

use mailchimp::{ApiRootType};
use mailchimp::resources::AutomationWorkflowResource;
use mailchimp::{ApiRoot, MailchimpApi, Automations, AutomationsFilter};

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
    pub fn create_stats<'a>(data: &AutomationWorkflowResource, account_name: &'a str) -> Self {
        let r_list_name = if let Some(r) = &data.get_recipients() {
            r.list_name.clone()
        } else {
            None
        };

        let s_title = if let Some(r) = &data.get_settings() {
            r.title.clone()
        } else {
            None
        };

        let mut rs_v = (0 as u64, 0 as u64, 0.0, 0 as u64, 0 as u64, 0.0, );

        if let Some(rp) = &data.get_report_summary() {
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
            status: data.get_status().clone(),
            emails_sent: data.get_emails_sent().clone(),
            recipients_list_name: r_list_name,
            title:s_title,
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
///
fn connect_mqtt<'a>(host: &'a str, user_name: &'a str, password: &'a str) -> mqtt::Client {
    let endpoint = String::from("tcp://") + host;
    let cli = mqtt::Client::new(endpoint.as_str()).unwrap_or_else(|err| {
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
    where T: serde::Serialize {
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
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let dc = env_mailchimp.next().unwrap().1;
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

    let mqtt_client = connect_mqtt(
        mqtt_settings.2.as_str(),
        mqtt_settings.0.as_str(),
        mqtt_settings.1.as_str(),
    );

    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&dc, &apk);
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

    disconnect(&mqtt_client);
}
