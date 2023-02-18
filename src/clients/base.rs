use reqwest::StatusCode;
use serde::de::DeserializeOwned;

#[doc(hidden)]
pub async fn get<T>(
    url: String,
    header: Option<(&str, &str)>,
    basic: Option<(&str, &str)>,
) -> Result<T, StatusCode>
where
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let mut call = client.get(&url);

    if let Some((key, value)) = header {
        call = call.header(key, value);
    }

    if let Some((user, pass)) = basic {
        call = call.basic_auth(user, Some(pass));
    }

    let resp = call.send().await;

    match resp {
        Ok(response) if !response.status().is_success() => Err(response.status()),
        Err(error) => Err(error.status().unwrap_or(StatusCode::BAD_REQUEST)),
        Ok(response) => {
            let content = response.json::<T>().await;

            match content {
                Ok(data) => Ok(data),
                Err(error) => {
                    println!("{:?}", error);
                    Err(StatusCode::BAD_REQUEST)
                }
            }
        }
    }
}
