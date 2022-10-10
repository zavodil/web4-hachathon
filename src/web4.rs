use crate::*;

use near_sdk::json_types::Base64VecU8;
use near_sdk::{env};
use std::collections::HashMap;

const STYLES_BODY: &str = include_str!("../res/style.css");

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    account_id: Option<AccountId>,
    path: String,
    params: Option<HashMap<String, String>>,
    query: Option<HashMap<String, Vec<String>>>,
    preloads: Option<HashMap<String, Web4Response>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Response {
    #[serde(rename = "contentType")]
    content_type: Option<String>,
    status: Option<u32>,
    body: Option<Base64VecU8>,
    #[serde(rename = "bodyUrl")]
    body_url: Option<String>,
    #[serde(rename = "preloadUrls")]
    preload_urls: Option<Vec<String>>,
}

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        let path = request.path;

        if path == "/robots.txt" {
            return Web4Response::plain_response("User-agent: *\nDisallow:".to_string());
        }

        if path == "/register" {
            return Web4Response::html_response(
                include_str!("../res/register.html")
                    .replace("%STYLESHEET%", &STYLES_BODY)
                    .replace("%CONTRACT_ID%", &env::current_account_id().to_string())
                    .replace("%NETWORK%", "testnet")
            );
        }

        let mut app_html = "".to_string();
        for (account_id, application_data) in self.get_applications(None, None) {
            if application_data.hidden == Some(false) {
                let mut youtube_url_text = "".to_string();

                if let Some(youtube_url) = application_data.youtube_url {
                    if !youtube_url.is_empty() {
                        youtube_url_text = format!("<a href=\"{}\">Youtube</a>&nbsp;|| ", youtube_url);
                    }
                };

                let text = format!("{}<br />{}<a href=\"{}\">Github</a>&nbsp;|| Contract: {}", application_data.description, youtube_url_text, application_data.github_url, application_data.contract_id);

                let winner_text = if let Some(reward) = application_data.reward {
                    format!(" [Prize: {} NEAR]", format_ynear(reward))
                } else {
                    "".to_string()
                };

                app_html = format!("{}<tr><td class=\"column-1\">{}</td><td class=\"column-2\">{}{}</td><td class=\"column-3\">{}</td></tr>", &app_html,
                                   text,
                                   account_id.to_string(),
                                   winner_text,
                                   application_data.contact_data
                );
            }
        }

        Web4Response::html_response(
            include_str!("../res/index.html")
                .replace("%STYLESHEET%", &STYLES_BODY)
                .replace("%REWARD_POOL%", &format_ynear(self.prize_pool))
                .replace("%DEADLINE%", &format_timestamp(self.deadline))
                .replace("%APPLICATIONS%", &app_html)
        )
    }
}

// format yNEAR value to human readable NEAR
fn format_ynear(value: u128) -> String {
    let value: f64 = (value / 1_000_000_000_000_000_000_000) as f64 / 1000f64;
    value.to_string()
}

fn format_timestamp(value: Option<Timestamp>) -> String {
    let date = if let Some(value) = value {
        format!("(new Date({}).toLocaleTimeString(\"en-US\"))", value)
    } else {
        "\"Not set\"".to_string()
    };
    format!("document.getElementById(\"deadline\").innerText={};", date)
}


impl Web4Response {
    pub fn html_response(text: String) -> Self {
        Self {
            content_type: Some(String::from("text/html; charset=UTF-8")),
            body: Some(text.into_bytes().into()),
            ..Default::default()
        }
    }

    pub fn plain_response(text: String) -> Self {
        Self {
            content_type: Some(String::from("text/plain; charset=UTF-8")),
            body: Some(text.into_bytes().into()),
            ..Default::default()
        }
    }

    pub fn preload_urls(urls: Vec<String>) -> Self {
        Self {
            preload_urls: Some(urls),
            ..Default::default()
        }
    }

    pub fn body_url(url: String) -> Self {
        Self {
            body_url: Some(url),
            ..Default::default()
        }
    }

    pub fn status(status: u32) -> Self {
        Self {
            status: Some(status),
            ..Default::default()
        }
    }
}
