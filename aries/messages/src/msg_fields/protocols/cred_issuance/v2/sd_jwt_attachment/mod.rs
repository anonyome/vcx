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
    fn test_sd_jwt_offer_format_serde() {
        let specifier = AttachmentFormatSpecifier {
            attach_id: "1".to_owned(),
            format: MaybeKnown::Known(OfferCredentialAttachmentFormatType::DidcommVcSdJwtOffer1_0),
        };
        let json = serde_json::to_value(&specifier).unwrap();
        assert_eq!(json["format"], json!("didcomm/vc+sd-jwt-offer@v1.0"));
        let deserialized: AttachmentFormatSpecifier<OfferCredentialAttachmentFormatType> =
            serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, specifier);
    }

    #[test]
    fn test_sd_jwt_request_format_serde() {
        let specifier = AttachmentFormatSpecifier {
            attach_id: "1".to_owned(),
            format: MaybeKnown::Known(
                RequestCredentialAttachmentFormatType::DidcommVcSdJwtRequest1_0,
            ),
        };
        let json = serde_json::to_value(&specifier).unwrap();
        assert_eq!(json["format"], json!("didcomm/vc+sd-jwt-request@v1.0"));
        let deserialized: AttachmentFormatSpecifier<RequestCredentialAttachmentFormatType> =
            serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, specifier);
    }

    #[test]
    fn test_sd_jwt_credential_format_serde() {
        let specifier = AttachmentFormatSpecifier {
            attach_id: "1".to_owned(),
            format: MaybeKnown::Known(IssueCredentialAttachmentFormatType::DidcommVcSdJwt1_0),
        };
        let json = serde_json::to_value(&specifier).unwrap();
        assert_eq!(json["format"], json!("didcomm/vc+sd-jwt@v1.0"));
        let deserialized: AttachmentFormatSpecifier<IssueCredentialAttachmentFormatType> =
            serde_json::from_value(json).unwrap();
        assert_eq!(deserialized, specifier);
    }
}
