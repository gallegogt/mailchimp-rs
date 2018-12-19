extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::MailchimpClient;
use std::collections::HashMap;

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let dc = env_mailchimp.next().unwrap().1;
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let client = MailchimpClient::new(&dc, &apk);

    // Ejemplo de como obtener todas la automatizaciones
    let account_automations = client.get_account_automations(HashMap::new());
    let mut last_automation_id = String::from("");

    match account_automations {
        Ok(automations) => {
            for w in &automations {
                let settings = w.get_settings();
                last_automation_id = w.get_id().clone();
                println!("Automatizacion");
                println!("ID                {:?}", w.get_id());
                println!("Título            {:?}", settings.title);
                println!("Emails Enviados   {:?}", w.get_emails_sent());
                println!("Resumen           {:?}", w.get_report_summary());
                println!("Fecha Inicio      {:?}", w.get_start_time());
                println!("Fecha de creacion {:?}", w.get_create_time());
                println!("Estado            {:?}", w.get_status());
                println!("Tracking          {:?}", w.get_tracking());
                println!("Disparadores      {:?}", w.get_trigger_settings());
                println!("Recipients        {:?}", w.get_recipients());
                println!("=============================================")
            }
        }
        Err(e) => println!("{:?}", e),
    };

    // Ejemplo de como obtener una automatización en especifiv¡co
    println!("\n\n");
    let workflow = client
        .get_automation_workflow_info(last_automation_id.as_str(), HashMap::new())
        .unwrap();
    println!(
        "Configuracion del workflow ID:{}  \n{:?}",
        last_automation_id,
        workflow.get_settings()
    );

    // Ejemplo de filtrado, en este caso es dfiltrado por el estado con valor "enviando"
    println!("\n\n");
    let mut filter = HashMap::new();
    filter.insert("status".to_string(), "sending".to_string());
    let filter_automations = client.get_account_automations(filter);

    match filter_automations {
        Ok(automations) => {
            for w in &automations {
                let settings = w.get_settings();
                println!("Automatizacion");
                println!("ID                {:?}", w.get_id());
                println!("Título            {:?}", settings.title);
                println!("Emails Enviados   {:?}", w.get_emails_sent());
                println!("Resumen           {:?}", w.get_report_summary());
                println!("Fecha Inicio      {:?}", w.get_start_time());
                println!("Fecha de creacion {:?}", w.get_create_time());
                println!("Estado            {:?}", w.get_status());
                println!("Tracking          {:?}", w.get_tracking());
                println!("Disparadores      {:?}", w.get_trigger_settings());
                println!("Recipients        {:?}", w.get_recipients());
                println!("=============================================")
            }
        }
        Err(e) => println!("{:?}", e),
    };
}
