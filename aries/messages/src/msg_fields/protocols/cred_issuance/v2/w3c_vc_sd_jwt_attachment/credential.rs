use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct W3cVcSdJwtCredentialAttachment {
    pub credential: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_w3c_vc_sd_jwt_credential_attachment() {
        let expected = json!({
            "credential": "eyJhbGciOiJFUzI1NiJ9.eyJfc2QiOi...~"
        });

        let deserialized: W3cVcSdJwtCredentialAttachment =
            serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(
            deserialized.credential,
            "eyJhbGciOiJFUzI1NiJ9.eyJfc2QiOi...~"
        );
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), expected);
    }
}
