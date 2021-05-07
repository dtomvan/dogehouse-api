#[macro_export]
macro_rules! request {
    ($url:ident) => {{
        let res = reqwest::get($url)
            .await
            .or_else(|_| return Err(crate::ErrorType::NoHTTP))?;
        let status = res.status();
        if let reqwest::StatusCode::OK = status {
            Ok(res)
        } else {
            Err(crate::ErrorType::NotFound(status.as_u16()))
        }
    }};
}

#[macro_export]
macro_rules! endpoint {
    ($(#[$meta:meta])*
     $endpoint:ident,$scheme:ty,$url:literal) => {
        $(#[$meta])*
        pub struct $endpoint;
        #[async_trait::async_trait]
        impl crate::endpoints::Endpoint for $endpoint {
            type Scheme = $scheme;
            async fn send() -> Result<Self::Scheme, crate::ErrorType> {
                let url = format!("{BASE_URL}{}", $url);
                let res = crate::request!(url)?;
                let text = res.text().await.or_else(|_| Err(crate::ErrorType::NoHTTP))?;
                serde_json::from_str::<$scheme>(text.as_str())
                    .or_else(|e| Err(crate::ErrorType::ParseError(e)))
            }
        }
    };
}
