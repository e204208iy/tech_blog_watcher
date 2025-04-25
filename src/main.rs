use reqwest::blocking::Client;
use rss::Channel;
use serde_json::json;
use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let notion_token = env::var("NOTION_TOKEN").expect("Missing NOTION_TOKEN");
    let db_id = env::var("NOTION_DB_ID").expect("Missing NOTION_DB_ID");
    let slack_webhook = env::var("SLACK_WEBHOOK_URL").expect("Missing SLACK_WEBHOOK_URL");

    let feed_urls = vec![
        "https://example.com/feed.xml", 
        "https://tech-blog.rust-lang.org/feed.xml",
    ];

    for feed_url in feed_urls {
        println!("🔍 Fetching: {}", feed_url);
        if let Ok(channel) = fetch_rss(feed_url) {
            for item in channel.items().iter().take(3) { // 最新3件だけ処理（好みに応じて）
                let title = item.title().unwrap_or("No title");
                let link = item.link().unwrap_or("No link");

                send_to_notion(title, link, &notion_token, &db_id);
                send_slack_notification(&slack_webhook, title, link);
            }
        }
    }
}

fn fetch_rss(url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn send_to_notion(title: &str, url: &str, token: &str, db_id: &str) {
    let client = Client::new();
    let res = client
        .post("https://api.notion.com/v1/pages")
        .header("Authorization", format!("Bearer {}", token))
        .header("Notion-Version", "2022-06-28")
        .header("Content-Type", "application/json")
        .json(&json!({
            "parent": { "database_id": db_id },
            "properties": {
                "Name": {
                    "title": [{
                        "text": {
                            "content": title
                        }
                    }]
                },
                "URL": {
                    "url": url
                }
            }
        }))
        .send()
        .unwrap();

    println!("✅ Notion登録: {} [{}]", title, res.status());
}

fn send_slack_notification(webhook_url: &str, title: &str, url: &str) {
    let client = Client::new();
    let payload = json!({ "text": format!("🆕 新着記事: {}\n{}", title, url) });

    let res = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .unwrap();

    println!("📢 Slack通知: {} [{}]", title, res.status());
}
