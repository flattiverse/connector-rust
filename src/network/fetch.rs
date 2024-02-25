use crate::network::ConnectError;

pub async fn get_text(uri: &str) -> Result<String, ConnectError> {
    match reqwest::Client::new().get(uri).send().await {
        Ok(response) => match response.text().await {
            Ok(response) => Ok(response),
            Err(e) => Err(ConnectError::Unknown(format!("{e:?}"))),
        },
        Err(e) => Err(ConnectError::Unknown(format!("{e:?}"))),
    }
}
