#[macro_export]
#[doc(hidden)]
macro_rules! request {
    ($url:ident) => {{
        use std::convert::TryFrom;

        let https = hyper_tls::HttpsConnector::new();
        let client = hyper::client::Client::builder()
            .build::<_, hyper::Body>(https);
        let ok_code = hyper::StatusCode::from_u16(200).unwrap();
        let res = client.get(hyper::Uri::try_from($url).unwrap())
            .await
            .or_else(|e| {println!("{}", e); Err(crate::ErrorType::NoHTTP("Could not send request"))})?;
        let status = res.status();
        if ok_code == status {
            Ok(res)
        } else {
            Err(crate::ErrorType::NotFound(status.as_u16()))
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! endpoint {
    ($scheme:ty, $url:ident) => {{
        let res = crate::request!($url)?;
        let res = crate::endpoint!(res;$scheme);
        res
    }};
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
                let res = crate::endpoint!(res;$scheme);
                res
            }
        }
    };
    ($res:ident;$scheme:ty) => {{
        let body = $res.into_body();
        let bytes = hyper::body::to_bytes(body)
            .await
            .or_else(|_| Err(crate::ErrorType::NoHTTP("Could not convert response into bytes.")))?;
        serde_json::from_str::<$scheme>(std::str::from_utf8(&bytes)
            .or_else(|_| Err(crate::ErrorType::NoHTTP("Could not read bytes from server.")))?)
            .or_else(|e| Err(crate::ErrorType::ParseError(e)))
    }};
}
