use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[doc(hidden)]
pub async fn get<T>(url: String, header: Option<(&str, &str)>) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();

    let mut call = client.get(url);

    if let Some((k, v)) = header {
        call = call.header(k, v)
    }

    let resp = call.send().await;

    match &resp {
        Ok(r) => {
            if r.status() != StatusCode::OK {
                return Err(r.status());
            }
        }
        Err(e) => {
            if e.is_status() {
                return Err(e.status().unwrap());
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let content = resp.unwrap().json::<T>().await;

    match content {
        Ok(s) => Ok(s),
        Err(e) => {
            println!("{:?}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}
