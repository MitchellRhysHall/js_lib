#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]

//! # js_lib
//!
//! The `js_lib` crate provides simple 'javascript-like' functions.
//! 
//! ## Making a http get request
//!
//! ```rust
//! use js_lib::fetch;
//! # async fn example() -> Result<(), js_lib::Error> {
//! let word = fetch("https://www.google.com/").await;
//! # Ok(())
//! # }
//! ```

/// A `Result` alias where the `Err` case is `js_lib::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that may occur.
///
/// Note: This is an enumumation of dependency crate errors
#[derive(Debug)]
pub enum Error {
    /// All networking errors which can occur.
    Network(reqwest::Error),
    /// All json parsing errors which can occur.
    ParseJson(serde_json::Error)
}

/// Fetches data from a url.
/// 
/// **NOTE**: This function makes a http GET request.
///
/// # Examples
///
/// ```rust
/// use js_lib::fetch;
/// # async fn example() -> Result<(), js_lib::Error> {
/// let word = fetch("https://www.google.com/").await;
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - there is a http request error
/// - the response has no body
pub async fn fetch(url: &str) -> Result<String> {
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Network(error))
        },
        Err(error) => Err(Error::Network(error))
    }
}

/// Deserializes a json string slice into type T.
///
/// # Examples
///
/// ```rust
/// use js_lib::from_json;
/// # async fn example() -> Result<(), js_lib::Error> {
/// let array = from_json::<Vec<String>>(r#"["1","2","3"]"#).unwrap();
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - there is an error parsing the json string slice
pub fn from_json<T>(json: &str) -> Result<T>
    where T: serde::de::DeserializeOwned {
    match serde_json::from_str::<T>(json) {
        Ok(data_struct) => Ok(data_struct),
        Err(error) => Err(Error::ParseJson(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_API: &str = "https://random-word-api.herokuapp.com/word";

    #[tokio::test(flavor = "multi_thread")]
    async fn fetch_api() {
        let result = fetch(TEST_API).await;
        assert!(result.is_ok());
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn deserialize_string() {
        let result = from_json::<Vec<String>>(r#"["1","2","3"]"#);
        assert!(result.is_ok());
    }
}
