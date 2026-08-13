use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct W3cVcSdJwtRequestAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_proof: Option<W3cVcSdJwtBindingProof>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct W3cVcSdJwtBindingProof {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub didcomm_signed_attachment: Option<DidcommSignedAttachmentProof>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DidcommSignedAttachmentProof {
    pub attachment_id: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_w3c_vc_sd_jwt_request_attachment() {
        let expected = json!({
            "binding_proof": {
                "didcomm_signed_attachment": {
                    "attachment_id": "attachment-0"
                }
            }
        });

        let deserialized: W3cVcSdJwtRequestAttachment =
            serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(
            deserialized
                .binding_proof
                .as_ref()
                .unwrap()
                .didcomm_signed_attachment
                .as_ref()
                .unwrap()
                .attachment_id,
            "attachment-0"
        );
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), expected);
    }

    #[test]
    fn test_w3c_vc_sd_jwt_request_attachment_no_binding() {
        let expected = json!({});
        let deserialized: W3cVcSdJwtRequestAttachment =
            serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(deserialized.binding_proof, None);
        assert_eq!(serde_json::to_value(&deserialized).unwrap(), expected);
    }
}
