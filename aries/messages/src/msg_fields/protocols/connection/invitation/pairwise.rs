use did_parser_nom::Did;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
use url::Url;

use super::InvitationContent;

pub type PairwiseInvitationContent = PwInvitationContent<Url>;
pub type PairwiseDidInvitationContent = PwInvitationContent<Did>;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(build_method(into = InvitationContent))]
#[builder(builder_type(vis = "pub"))]
pub struct PwInvitationContent<T>
where
    PwInvitationContent<T>: Into<InvitationContent>,
{
    pub label: String,
    pub recipient_keys: Vec<String>,
    #[builder(default)]
    #[serde(default)]
    pub routing_keys: Vec<String>,
    pub service_endpoint: T,
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::{
        decorators::timing::tests::make_extended_timing, misc::test_utils,
        msg_fields::protocols::connection::invitation::InvitationDecorators,
        msg_types::connection::ConnectionTypeV1_0,
    };

    #[test]
    fn test_minimal_conn_invite_pw() {
        let label = "test_pw_invite_label";
        let recipient_keys = vec!["test_recipient_key".to_owned()];
        let service_endpoint = "https://dummy.dummy/dummy";

        let content = InvitationContent::builder_pairwise()
            .label(label.to_owned())
            .recipient_keys(recipient_keys.clone())
            .service_endpoint(service_endpoint.parse().unwrap())
            .build();

        let decorators = InvitationDecorators::default();

        let expected = json!({
            "label": label,
            "recipientKeys": recipient_keys,
            "routingKeys": [],
            "serviceEndpoint": service_endpoint,
        });

        test_utils::test_msg(
            content,
            decorators,
            ConnectionTypeV1_0::Invitation,
            expected,
        );
    }

    #[test]
    fn test_extended_conn_invite_pw() {
        let label = "test_pw_invite_label";
        let recipient_keys = vec!["test_recipient_key".to_owned()];
        let routing_keys = vec!["test_routing_key".to_owned()];
        let service_endpoint = "https://dummy.dummy/dummy";

        let content = InvitationContent::builder_pairwise()
            .label(label.to_owned())
            .recipient_keys(recipient_keys.clone())
            .routing_keys(routing_keys.clone())
            .service_endpoint(service_endpoint.parse().unwrap())
            .build();

        let decorators = InvitationDecorators::builder()
            .timing(make_extended_timing())
            .build();

        let expected = json!({
            "label": label,
            "recipientKeys": recipient_keys,
            "routingKeys": routing_keys,
            "serviceEndpoint": service_endpoint,
            "~timing": decorators.timing
        });

        test_utils::test_msg(
            content,
            decorators,
            ConnectionTypeV1_0::Invitation,
            expected,
        );
    }

    #[test]
    fn test_minimal_conn_invite_pw_did() {
        let label = "test_pw_invite_label";
        let recipient_keys = vec!["test_recipient_key".to_owned()];
        let service_endpoint = "did:sov:123456789abcdefghi1234";

        let content = InvitationContent::builder_pairwise_did()
            .label(label.to_owned())
            .recipient_keys(recipient_keys.clone())
            .service_endpoint(service_endpoint.parse().unwrap())
            .build();

        let decorators = InvitationDecorators::default();

        let expected = json!({
            "label": label,
            "recipientKeys": recipient_keys,
            "routingKeys": [],
            "serviceEndpoint": service_endpoint,
        });

        test_utils::test_msg(
            content,
            decorators,
            ConnectionTypeV1_0::Invitation,
            expected,
        );
    }

    #[test]
    fn test_extended_conn_invite_pw_did() {
        let label = "test_pw_invite_label";
        let recipient_keys = vec!["test_recipient_key".to_owned()];
        let routing_keys = vec!["test_routing_key".to_owned()];
        let service_endpoint = "did:sov:123456789abcdefghi1234";

        let content = InvitationContent::builder_pairwise_did()
            .label(label.to_owned())
            .recipient_keys(recipient_keys.clone())
            .routing_keys(routing_keys.clone())
            .service_endpoint(service_endpoint.parse().unwrap())
            .build();

        let decorators = InvitationDecorators::builder()
            .timing(make_extended_timing())
            .build();

        let expected = json!({
            "label": label,
            "recipientKeys": recipient_keys,
            "routingKeys": routing_keys,
            "serviceEndpoint": service_endpoint,
            "~timing": decorators.timing
        });

        test_utils::test_msg(
            content,
            decorators,
            ConnectionTypeV1_0::Invitation,
            expected,
        );
    }
}
