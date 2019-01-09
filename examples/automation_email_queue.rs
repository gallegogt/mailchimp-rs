extern crate dotenv;
extern crate mailchimp;

use dotenv::dotenv;
use std::env;

use mailchimp::Automations;
use mailchimp::{AutomationsFilter, MailchimpApi};

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
    for workflow in automations.iter(AutomationsFilter::default()) {
        println!(
            "\nAutomation Workflow ID: {:?}  \n Title: {:?}",
            workflow.id,
            workflow.settings.as_ref().unwrap().title
        );

        let emails_resp = workflow.get_workflow_emails();

        match emails_resp {
            Ok(workflow_emails) => {
                for e in workflow_emails {
                    println!("\nWorkflow Emails");
                    println!("ID   {:?}", e.id);
                    println!("Emails Enviados   {:?}", e.emails_sent.as_ref().unwrap());
                    println!("Fecha Inicio      {:?}", e.start_time.as_ref().unwrap());
                    println!("Fecha de creacion {:?}", e.create_time.as_ref().unwrap());
                    println!("Resumen           {:?}", e.report_summary.as_ref().unwrap());

                    for aeq in e.get_email_queue() {
                        println!("\nAutomation Email Queue");
                        println!("{:?}", aeq);
                    }
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
