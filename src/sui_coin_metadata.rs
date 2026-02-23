pub struct SuiCoinMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub description: String,
    pub icon_url: Option<String>
}

impl From<sui_sdk::rpc_types::SuiCoinMetadata> for SuiCoinMetadata {
    fn from(metadata: sui_sdk::rpc_types::SuiCoinMetadata) -> Self {
        Self {
            name: metadata.name,
            symbol: metadata.symbol,
            decimals: metadata.decimals,
            description: metadata.description,
            icon_url: metadata.icon_url
        }
    }
}
