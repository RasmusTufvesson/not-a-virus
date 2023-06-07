use serenity::http::Http;
use serenity::model::webhook::Webhook;

pub struct Hook {
    http: Http,
    webhook: Webhook,
    name: String,
    icon_url: String,
}

impl Hook {
    pub async fn new(url: &str, name: &str, icon_url: &str) -> Self {
        let http = Http::new("");
        let webhook = Webhook::from_url(&http, url)
            .await
            .expect("Could not create webhook");
        Self {
            http,
            webhook,
            name: name.to_string(),
            icon_url: icon_url.to_string(),
        }
    }

    pub async fn send(&self, message: &str) {
        self.webhook.execute(&self.http, false, |w| w.content(message).username(&self.name).avatar_url(&self.icon_url))
            .await
            .expect("Could not send message");
    }
}