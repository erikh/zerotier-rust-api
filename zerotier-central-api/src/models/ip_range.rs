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
pub struct IpRange {
    #[serde(rename = "ipRangeStart", skip_serializing_if = "Option::is_none")]
    pub ip_range_start: Option<String>,
    #[serde(rename = "ipRangeEnd", skip_serializing_if = "Option::is_none")]
    pub ip_range_end: Option<String>,
}

impl IpRange {
    pub fn new() -> IpRange {
        IpRange {
            ip_range_start: None,
            ip_range_end: None,
        }
    }
}


