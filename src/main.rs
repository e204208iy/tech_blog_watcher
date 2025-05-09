mod feeds;

use feeds::get_feed_sources;

use rss::Channel;
use chrono::{DateTime, Utc, Duration};
use reqwest;
use dotenv::dotenv;
use serde_json::json;
use std::{env, fs, collections::HashSet};
use reqwest::blocking::Client;

const NOTIFIED_URLS_FILE: &str = "notified_urls.txt"; // 保存ファイル

fn main() {
    dotenv().ok();
    let slack_webhook = env::var("SLACK_WEBHOOK_URL").expect("Missing SLACK_WEBHOOK_URL");

    let feed_sources = get_feed_sources();

    let mut notified_urls = load_notified_urls();

    for (company, feed_url) in feed_sources {
        println!("🔍 Fetching: {} ({})", company, feed_url);
        if let Ok(channel) = fetch_rss(feed_url) {
            for item in channel.items() {
                let title = item.title().unwrap_or("No title");
                let url = item.link().unwrap_or("No link");

                // pubDateがある場合
                if let Some(pub_date_str) = item.pub_date() {
                    if let Ok(pub_date) = DateTime::parse_from_rfc2822(pub_date_str) {
                        let now = Utc::now();
                        let pub_date_utc = pub_date.with_timezone(&Utc);

                        if now - pub_date_utc < Duration::hours(24) {
                            if notified_urls.insert(url.to_string()) {
                                send_slack_notification(&slack_webhook, company, title, url);
                            } else {
                                println!("✅ すでに通知済み: {}", title);
                            }
                        } else {
                            println!("🕰️ 古い記事なのでスキップ: {}", title);
                        }
                    } else {
                        println!("⚠️ pubDateパース失敗 → リンク比較に切り替え: {}", pub_date_str);
                        notify_if_new(&mut notified_urls, &slack_webhook, company, title, url);
                    }
                } else {
                    println!("⚠️ pubDateなし → リンク比較に切り替え: {}", title);
                    notify_if_new(&mut notified_urls, &slack_webhook, company, title, url);
                }
            }
        }
    }

    save_notified_urls(&notified_urls);
}

// RSS取得
fn fetch_rss(url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

// Slack通知
fn send_slack_notification(webhook_url: &str, company: &str, title: &str, url: &str) {
    let client = Client::new();
    let payload = json!({ "text": format!("🆕 新着記事: {}\n{}\n{}", company, title, url) });

    let res = client
        .post(webhook_url)
        .json(&payload)
        .send()
        .unwrap();

    println!("📢 Slack通知: {} [{}]", title, res.status());
}

// 通知するか判断（リンク重複チェック）
fn notify_if_new(notified_urls: &mut HashSet<String>, slack_webhook: &str, company: &str, title: &str, url: &str) {
    if notified_urls.insert(url.to_string()) {
        send_slack_notification(slack_webhook, company, title, url);
    } else {
        println!("✅ すでに通知済み（pubDateなし）: {}", title);
    }
}

// 保存ファイルからURLリスト読み込み
fn load_notified_urls() -> HashSet<String> {
    match fs::read_to_string(NOTIFIED_URLS_FILE) {
        Ok(content) => content.lines().map(|s| s.to_string()).collect(),
        Err(_) => HashSet::new(),
    }
}

// URLリストを保存
fn save_notified_urls(urls: &HashSet<String>) {
    let content = urls.iter().cloned().collect::<Vec<_>>().join("\n");
    fs::write(NOTIFIED_URLS_FILE, content).expect("Failed to save notified URLs");
}