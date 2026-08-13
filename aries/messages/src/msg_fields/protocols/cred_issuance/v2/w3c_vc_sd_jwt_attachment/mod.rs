pub mod credential;
pub mod offer;
pub mod request;

#[cfg(test)]
mod tests {
    use serde_json::json;
    use shared::maybe_known::MaybeKnown;

    use super::super::{
        issue_credential::IssueCredentialAttachmentFormatType,
        offer_credential::OfferCredentialAttachmentFormatType,
        request_credential::RequestCredentialAttachmentFormatType,
    };
    use crate::msg_fields::protocols::common::attachment_format_specifier::AttachmentFormatSpecifier;

    #[test]
    fn test_w3c_vc_sd_jwt_offer_format_serde() {
        let specifier = AttachmentFormatSpecifier {
            attach_id: "1".to_owned(),
            format: MaybeKnown::Known(
                OfferCredentialAttachmentFormatType::DidcommW3cVcSdJwtOffer1_0,
            ),
        };
        let json = serde_json::to_value(&specifier).unwrap();
        assert_eq!(json["format"], json!("didcomm/w3c-vc-sd-jwt-offer@v1.0"));
        let deserialized: AttachmentFormatSpecifier<OfferCredentialAttachmentFormatType> =
            serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, specifier);
    }

    #[test]
    fn test_w3c_vc_sd_jwt_request_format_serde() {
        let specifier = AttachmentFormatSpecifier {
            attach_id: "1".to_owned(),
            format: MaybeKnown::Known(
                RequestCredentialAttachmentFormatType::DidcommW3cVcSdJwtRequest1_0,
            ),
        };
        let json = serde_json::to_value(&specifier).unwrap();
        assert_eq!(json["format"], json!("didcomm/w3c-vc-sd-jwt-request@v1.0"));
        let deserialized: AttachmentFormatSpecifier<RequestCredentialAttachmentFormatType> =
            serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, specifier);
    }

    #[test]
    fn test_w3c_vc_sd_jwt_credential_format_serde() {
        let specifier = AttachmentFormatSpecifier {
            attach_id: "1".to_owned(),
            format: MaybeKnown::Known(IssueCredentialAttachmentFormatType::DidcommW3cVcSdJwt1_0),
        };
        let json = serde_json::to_value(&specifier).unwrap();
        assert_eq!(json["format"], json!("didcomm/w3c-vc-sd-jwt@v1.0"));
        let deserialized: AttachmentFormatSpecifier<IssueCredentialAttachmentFormatType> =
            serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, specifier);
    }
}
