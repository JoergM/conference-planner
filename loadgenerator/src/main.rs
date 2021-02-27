use log::{error, info};
use rand::Rng;
use std::time;

/* A very simple load generator. It is just a synchronous loop with a random choice of target and a certain wait between calls.
 */

fn main() {
    env_logger::init();

    let target_urls = vec![
        "http://frontend-proxy:8080/",
        "http://frontend-proxy:8080/speakers/",
        "http://frontend-proxy:8080/schedule/",
        "http://frontend-proxy:8080/sessions/",
    ];

    loop {
        //select random url
        let mut rng = rand::thread_rng();
        let url_idx = rng.gen_range(0..target_urls.len());
        let url = target_urls[url_idx];

        //call url
        let result = reqwest::blocking::get(url);
        if result.is_ok() {
            let resp = result.unwrap();
            info!("Called {} and got return code {}", &url, resp.status());
        } else {
            let error = result.unwrap_err();
            error!("{}", error);
        }

        //sleep a little
        let delay = time::Duration::from_millis(500);
        std::thread::sleep(delay)
        //repeat
    }
}
