use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct W3cVcSdJwtOfferAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_method: Option<W3cVcSdJwtBindingMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectively_disclosable_claims: Option<Vec<String>>,
    pub credential: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct W3cVcSdJwtBindingMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didcomm_signed_attachment: Option<DidcommSignedAttachmentOfferMethod>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DidcommSignedAttachmentOfferMethod {
    pub algs_supported: Vec<String>,
    pub did_methods_supported: Vec<String>,
    pub nonce: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_w3c_vc_sd_jwt_offer_attachment_full() {
        let expected = json!({
            "binding_required": true,
            "binding_method": {
                "didcomm_signed_attachment": {
                    "algs_supported": ["ES256", "EdDSA"],
                    "did_methods_supported": ["key", "jwk"],
                    "nonce": "b19439b0-4dc9-4c28-b796-99d17034fb5c"
                }
            },
            "selectively_disclosable_claims": [
                "$.credentialSubject.degree.name",
                "$.credentialSubject.degree.type"
            ],
            "credential": {
                "@context": ["https://www.w3.org/ns/credentials/v2"],
                "type": ["VerifiableCredential"],
                "issuer": "did:key:z6MkodKV3mnjQQMB9jhMZtKD9Sm75ajiYq51JDLuRSPZTXrr",
                "credentialSubject": {}
            }
        });

        let deserialized: W3cVcSdJwtOfferAttachment =
            serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(deserialized.binding_required, Some(true));
        assert_eq!(
            deserialized
                .binding_method
                .as_ref()
                .unwrap()
                .didcomm_signed_attachment
                .as_ref()
                .unwrap()
                .nonce,
            "b19439b0-4dc9-4c28-b796-99d17034fb5c"
        );
        assert_eq!(
            deserialized.selectively_disclosable_claims,
            Some(vec![
                "$.credentialSubject.degree.name".to_owned(),
                "$.credentialSubject.degree.type".to_owned()
            ])
        );
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), expected);
    }

    #[test]
    fn test_w3c_vc_sd_jwt_offer_attachment_minimal() {
        let expected = json!({
            "credential": {
                "@context": ["https://www.w3.org/ns/credentials/v2"],
                "type": ["VerifiableCredential"]
            }
        });

        let deserialized: W3cVcSdJwtOfferAttachment =
            serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(deserialized.binding_required, None);
        assert_eq!(deserialized.binding_method, None);
        assert_eq!(deserialized.selectively_disclosable_claims, None);
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), expected);
    }
}
