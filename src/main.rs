extern crate imap;
extern crate native_tls;

// use imap::types::ZeroCopy;
use serde::Deserialize;
use std::fs::File;
// use std::io::Write;

#[derive(Debug, Deserialize)]
struct Config {
    email_user: String,
    email_pass: String,
    imap_server: String,
    //output_dir: String,
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let config_file = File::open("src/config.json").expect("Impossible d'ouvrir le fichier de configuration");
    let config: Config = serde_json::from_reader(config_file).expect("Impossible de désérialiser le JSON");
    // println!("{:?}", config);

      // Créez un client IMAP
   let domain = config.imap_server;
   let tls = native_tls::TlsConnector::builder().build().unwrap();
   let client = imap::connect((domain.clone(), 993), domain.clone(), &tls).unwrap();

   // Authentifiez-vous avec le serveur IMAP
   let mut imap_session = client.login(config.email_user, config.email_pass).unwrap();

   // Sélectionnez la boîte de réception
   imap_session.select("INBOX").unwrap();


    // Récupérez le dernier message
    let mailbox = imap_session.select("INBOX");
    let messages = imap_session.fetch(mailbox.exists().to_string(), "BODY[]");
    //let last_message = messages.last().unwrap();
    println!("{:?}", messages);
    /*let last_msg = inbox.fetch(inbox.exists()?.to_string(), "(RFC822)")
    .await?
    .next()
    .ok_or_else(|| imap::error::Error::Bad("Aucun message trouvé".to_string()))?;

    // Récupérez le contenu du message
    if let Some(msg) = last_msg {
        let body = String::from_utf8_lossy(msg.envelope()?.message.to_vec().as_slice());
        let subject = msg.envelope()?.subject.unwrap_or_default();
    }

    // Récupérez le contenu du message */
    /*if let Some(msg) = last_msg {
        let body = String::from_utf8_lossy(msg.envelope()?.message.to_vec().as_slice());
        let subject = msg.envelope()?.subject.unwrap_or_default();

        // Enregistrez le message en format markdown
        let markdown_content = format!("# {}\n\n{}", subject, body);
        let output_file_path = format!("{}/{}.md", config.output_dir, subject);
        let mut output_file = File::create(&output_file_path)?;

        output_file.write_all(markdown_content.as_bytes())?;

        // Effacez le message
        client.store(&format!("{}", msg.message), "+FLAGS", "(\\Deleted)").await?;
        client.expunge().await?;
    }*/
    Ok(())
}