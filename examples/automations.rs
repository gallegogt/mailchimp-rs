extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::types::RecipientType;
use mailchimp::MailchimpApi;
use mailchimp::{Automations, AutomationsFilter};
use std::collections::HashMap;

fn main() {
    // Inicializando el dotenv
    dotenv().ok();
    // Obteniendo las variables de entornos con las credenciales de
    // mailchimp
    let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
    let apk = env_mailchimp.next().unwrap().1;
    // Inicializando el API, con las credenciales
    let api = MailchimpApi::new(&apk);

    // Ejemplo de como obtener todas la automatizaciones
    let automations = Automations::new(api);
    let mut last_automation_id = String::from("");

    for w in automations.iter(AutomationsFilter::default()) {
        let settings = w.settings.as_ref().unwrap();
        last_automation_id = w.id.as_ref().unwrap().to_string();
        println!("Automatizacion");
        println!("ID                {:?}", w.id);
        println!("Título            {:?}", settings.title);
        println!("Emails Enviados   {:?}", w.emails_sent);
        println!("Resumen           {:?}", w.report_summary);
        println!("Fecha Inicio      {:?}", w.start_time);
        println!("Fecha de creacion {:?}", w.create_time);
        println!("Estado            {:?}", w.status);
        println!("Tracking          {:?}", w.tracking);
        println!("Disparadores      {:?}", w.trigger_settings);
        println!("Recipients        {:?}", w.recipients);
        println!("=============================================")
    }

    // Ejemplo de como obtener una automatización en especifiv¡co
    println!("\n\n");
    let workflow = automations
        .get_automation_workflow_info(last_automation_id.as_str(), HashMap::new())
        .unwrap();
    println!(
        "Configuracion del workflow ID:{}  \n{:?}",
        last_automation_id, workflow.settings
    );

    // Ejemplo de filtrado, en este caso es dfiltrado por el estado con valor "enviando"
    println!("\n\n");
    let mut filter = HashMap::new();
    filter.insert("status".to_string(), "sending".to_string());

    for w in automations.iter(AutomationsFilter {
        status: Some("sending".to_string()),
        fields: None,
        exclude_fields: None,
        count: Some(50),
        offset: Some(0),
        before_send_time: None,
        since_send_time: None,
        before_create_time: None,
        since_create_time: None,
    }) {
        let settings = w.settings.as_ref().unwrap();
        println!("Automatizacion");
        println!("ID                {:?}", w.id);
        println!("Título            {:?}", settings.title);
        println!("Emails Enviados   {:?}", w.emails_sent);
        println!("Resumen           {:?}", w.report_summary);
        println!("Fecha Inicio      {:?}", w.start_time);
        println!("Fecha de creacion {:?}", w.create_time);
        println!("Estado            {:?}", w.status);
        println!("Tracking          {:?}", w.tracking);
        println!("Disparadores      {:?}", w.trigger_settings);
        println!("Recipients        {:?}", w.recipients);
        println!("=============================================")
    }

    // ==================== Workflow Emails ========== ====
    let emails_resp = workflow.get_workflow_emails();

    match emails_resp {
        Ok(workflow_emails) => {
            for e in workflow_emails {
                println!("\nWorkflow Emails");
                println!("ID   {:?}", e.id);
                println!("Emails Enviados   {:?}", e.emails_sent.as_ref().unwrap());
                println!("Fecha Inicio      {:?}", e.start_time.as_ref().unwrap());
                println!("Fecha de creacion {:?}", e.create_time.as_ref().unwrap());
                println!(
                    "Recipients        {:?}",
                    e.recipients.as_ref().unwrap_or(&RecipientType::default())
                );
                println!("Resumen           {:?}", e.report_summary.as_ref().unwrap());
            }
        }
        Err(e) => println!("{:?}", e),
    }

    // ============= Workflow Emails Get Info =======================
    let we_info = workflow.get_automation_workflow_email_info("000da526de");

    match we_info {
        Ok(we) => {
            println!("\nWorkflow Emails");
            println!("ID   {:?}", we.id.unwrap_or("".to_string()));
            println!(
                "Emails Enviados   {:?}",
                we.emails_sent.as_ref().unwrap_or(&0)
            );
            println!("Fecha Inicio      {:?}", we.start_time.as_ref().unwrap());
            println!("Fecha de creacion {:?}", we.create_time.as_ref().unwrap());
            println!("Recipients        {:?}", we.recipients.as_ref().unwrap());
            println!(
                "Resumen           {:?}",
                we.report_summary.as_ref().unwrap()
            );
        }
        Err(e) => println!("we_info {:?}", e),
    }
}
