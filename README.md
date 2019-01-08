# Mailchimp API

### Biblioteca de desarrollo para acceder al API de [Mailchimp](https://developer.mailchimp.com), utilizando como lenguaje de programación Rust

NOTA: Biblioteca en desarrollo, en la sección "**Estado de implementación**" te muestro que he implementado y que no
---

### ✅ Ejemplo de como puedes extraer todas las automatizaciones

A continuación te muestro un ejemplo de código para que puedas extraer todas las automatizaciones creadas en tu Mailchimp

Para este ejemplo uso las siguientes dependencias:

```toml
[dependencies]
dotenv = "^0.13"
mailchimp = "0.1"
```

También he creado un archivo .env con las credenciales para el acceso a mailchimp. A continuación te pongo un ejemplo del archivo .env

```
MAILCHIMP_API_KEY="<API_KEY>"
```

Finalmente el código de ejemplo para visualizar las automatizaciones creadas en tu mailchimp

```rust
  extern crate dotenv;
  extern crate mailchimp;

  use dotenv::dotenv;
  use std::env;

  use mailchimp::MailchimpApi;
  use mailchimp::{Automations, AutomationsFilter};
  use std::collections::HashMap;

  fn main() {
      // Inicializando el dotenv
      dotenv().ok();
      // Obteniendo las variables de entornos con las credenciales de
      // mailchimp
      let mut env_mailchimp = env::vars().filter(|e| e.0.to_string().contains("MAILCHIMP_"));
      // Inicializando el API, con las credenciales
      let apk = env_mailchimp.next().unwrap().1;
      // Inicializando el API, con las credenciales
      let api = MailchimpApi::new(&apk);

      // Ejemplo de como obtener todas la automatizaciones
      let automations = Automations::new(api);
      let mut last_automation_id = String::from("");

      for w in automations.iter(AutomationsFilter::default()) {
          let settings = w.get_settings().as_ref().unwrap();
          last_automation_id = w.get_id().as_ref().unwrap().to_string();
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
```

###Puedes seguir viendo mas ejemplos en la carpeta examples

# Estado de la implementación

A continuación te presento un listado donde iré actualizando a la medida de lo posible las diferentes los endpoints soportados por la biblioteca

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
      * 🔘 View queued subscribers for an automated email
      * 🔘 View specific subscriber in email queue

  * **Removed Subscribers**
    * 🔘 Remove subscriber from a workflow
    * 🔘 View all subscribers removed from a workflow


### List
  * 🔘 Create a new list
  * 🔘 Batch sub/unsub list members
  * ✅ Get information about all lists
  * ✅ Get information about a specific list

    * 🔘 **Abuse Reports**
    * 🔘 **Activity**
    * 🔘 **Clients**
    * 🔘 **Growth History**
    * 🔘 **Interest Categories**
    * 🔘 **Locations**
    * 🔘 **Members**
    * 🔘 **Merge Fields**
    * 🔘 **Segments**
    * 🔘 **Signup Forms**
    * 🔘 **Webhooks**
    * 🔘 **Locations**

### Campaigns
  * 🔘 Create a new campaign
  * ✅ Get all campaigns
  * ✅ Get information about a specific campaign
  * 🔘 Update the settings for a campaign
  * 🔘 Delete a campaign
  * 🔘 Cancel a campaign
  * 🔘 Resend a campaign
  * 🔘 Pause an RSS-Driven campaign
  * 🔘 Replicate a campaign
  * 🔘 Resume an RSS-Driven campaign
  * 🔘 Schedule a campaign
  * 🔘 Send a campaign
  * 🔘 Send a test email
  * 🔘 Unschedule a campaign

  * 🔘 **Content**
  * 🔘 **Feedback**
  * 🔘 **Send Checklist**

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
