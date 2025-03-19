use std::fmt::Error;

pub async fn get_series() {
    // TODO: get series upcomming dates from tmdb
    //
    let body = reqwest::get("https://www.rust-lang.org")
        .await?
        .text()
        .await?;

    println!("body = {body:?}");
}
