pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

#[allow(unused_imports)]
use solana_security_txt::security_txt;

declare_id!("8dGFLiUiUjJzQjMcptHuiLEVcGJjc2HTn6CQnAXSxxUF");

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Anchor Cowshit Test",
    project_url: "https://test.test",
    contacts: "email:info@test.test, twitter:@ProgSecTest",
    policy: "https://github.com/ImmortanJoeMomma/test-repo-anchor/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/ImmortanJoeMomma/test-repo-anchor"
}

#[program]
pub mod cowshit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
