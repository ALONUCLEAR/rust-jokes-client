mod joke;
use joke::Joke;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // "https://v2.jokeapi.dev/joke/Any"
        let (joke, error) = query_jokes("http://localhost:1234/jokes").await;

        if error.len() > 2 {
            println!("Error - {error}");
        } else {
            joke.ferris_delivery();
        }
    });
}

async fn query_jokes(gateway_url: &str) -> (Joke, String) {
    let basic_res = reqwest::get(gateway_url).await;

    let response = match basic_res {
        Ok(res) => (json_to_joke(res.text().await.unwrap()), "".to_owned()),
        Err(error) => (Joke::empty(), error.to_string()),
    };

    return response;
}

fn json_to_joke(raw_json: String) -> Joke {
    let parsed_json: serde_json::Value =
        serde_json::from_str(raw_json.as_str()).expect("JSON was not well-formatted");

    return Joke::new(parsed_json);
}
