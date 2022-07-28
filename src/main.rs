use roux::Subreddit;

use serenity::{http::Http, json::Value, model::channel::Embed, model::webhook::Webhook};

mod mal;
use mal::Mal;

#[tokio::main]
async fn main() {
    let webhook_url =
        std::env::var("WEBHOOK_URL").expect("Environment variable 'WEBHOOK_URL' not set!");

    let http = Http::new("");
    let webhook = Webhook::from_url(&http, &webhook_url).await.unwrap();

    let mal = Mal::new().unwrap();

    let mut known_posts: Vec<String> = Vec::new();
    let mut known_posts_new: Vec<String>;

    println!("Weeb Machine started - All systems nominal  ⚙️");

    loop {
        known_posts_new = Vec::new();
        let mut embeds: Vec<Value> = Vec::new();

        let subreddit = Subreddit::new("anime");
        if let Ok(posts) = subreddit.latest(100, None).await {
            for post in posts.data.children {
                if post.data.link_flair_text.as_deref() != Some("Episode") {
                    continue;
                }

                if !known_posts.contains(&post.data.id) {
                    println!("New post found: {}", post.data.id);
                    let image_url = mal
                        .return_first_image_url(&post.data.selftext)
                        .await
                        .unwrap_or_else(|| "".to_string());

                    let embed = Embed::fake(|e| {
                        e.title(post.data.title)
                            .url(format!("https://reddit.com{}", post.data.permalink))
                            .image(image_url)
                    });

                    embeds.push(embed);
                    known_posts_new.push(post.data.id);
                } else {
                    println!("Known post: {} - Ignored", post.data.id);
                }
            }
        }

        // Webhooks can only have 10 embeds, so we need to make multiple posts
        for start in (0..embeds.len()).step_by(10) {
            let end = (start + 10).clamp(start + 1, embeds.len());
            let slice: Vec<Value> = embeds[start..end].to_vec();
            let _ = webhook.execute(&http, false, |w| w.embeds(slice)).await;
        }

        known_posts = known_posts_new;

        println!("Done - Next run in 10 Minutes");

        let wait_time = std::time::Duration::from_secs(600);
        std::thread::sleep(wait_time);
    }
}
