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
pub struct Ipv6AssignMode {
    #[serde(rename = "6plane", skip_serializing_if = "Option::is_none")]
    pub var_6plane: Option<bool>,
    #[serde(rename = "rfc4193", skip_serializing_if = "Option::is_none")]
    pub rfc4193: Option<bool>,
    #[serde(rename = "zt", skip_serializing_if = "Option::is_none")]
    pub zt: Option<bool>,
}

impl Ipv6AssignMode {
    pub fn new() -> Ipv6AssignMode {
        Ipv6AssignMode {
            var_6plane: None,
            rfc4193: None,
            zt: None,
        }
    }
}


