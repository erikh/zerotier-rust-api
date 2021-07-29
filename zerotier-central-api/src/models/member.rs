/*
 * ZeroTier Central API
 *
 * ZeroTier Central Network Management Portal API.<p>All API requests must have an API token header specified in the <code>Authorization: Bearer xxxxx</code> format.  You can generate your API key by logging into <a href=\"https://my.zerotier.com\">ZeroTier Central</a> and creating a token on the Account page.</p><p>eg. <code>curl -X GET -H \"Authorization: bearer xxxxx\" https://my.zerotier.com/api/v1/network</code></p>
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Member {
    /// concatenation of network ID and member ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "clock", skip_serializing_if = "Option::is_none")]
    pub clock: Option<i64>,
    #[serde(rename = "networkId", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// ZeroTier ID of the member
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "controllerId", skip_serializing_if = "Option::is_none")]
    pub controller_id: Option<String>,
    /// Whether or not the member is hidden in the UI
    #[serde(rename = "hidden", skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    /// User defined name of the member
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User defined description of the member
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<crate::models::MemberConfig>>,
    /// Last seen time of the member
    #[serde(rename = "lastOnline", skip_serializing_if = "Option::is_none")]
    pub last_online: Option<i64>,
    /// IP address the member last spoke to the controller via
    #[serde(rename = "physicalAddress", skip_serializing_if = "Option::is_none")]
    pub physical_address: Option<String>,
    /// ZeroTier version the member is running
    #[serde(rename = "clientVersion", skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,
    /// ZeroTier protocol version
    #[serde(rename = "protocolVersion", skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<i32>,
    /// Whether or not the client version is new enough to support the rules engine (1.4.0+)
    #[serde(rename = "supportsRulesEngine", skip_serializing_if = "Option::is_none")]
    pub supports_rules_engine: Option<bool>,
}

impl Member {
    pub fn new() -> Member {
        Member {
            id: None,
            clock: None,
            network_id: None,
            node_id: None,
            controller_id: None,
            hidden: None,
            name: None,
            description: None,
            config: None,
            last_online: None,
            physical_address: None,
            client_version: None,
            protocol_version: None,
            supports_rules_engine: None,
        }
    }
}


