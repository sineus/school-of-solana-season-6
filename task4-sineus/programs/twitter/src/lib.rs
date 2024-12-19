// The task is to add code wherever you find TODO or todo!().
// Once you complete all TODOs, make sure to deleted all `todo!()` macros.
// Try running the tests using `anchor test` command and start debugging ;-)

// We recommend completing the code in the following order:
// 1. initialize_tweet.rs
// 2. add_reaction.rs
// 3. add_comment.rs
// 4. remove_reaction.rs

use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod states;

declare_id!("F6NKeaoPbchYnbcJZ5YSAqfMcHuP7GLExTuDK3qmgtgW");

#[program]
pub mod twitter {

    use super::*;

    pub fn initialize(ctx: Context<InitializeTweet>, topic: String, content: String) -> Result<()> {
        initialize_tweet(ctx, topic, content)
    }
    pub fn like_tweet(ctx: Context<AddReactionContext>) -> Result<()> {
        add_reaction(ctx, states::ReactionType::Like)
    }
    pub fn dislike_tweet(ctx: Context<AddReactionContext>) -> Result<()> {
        add_reaction(ctx, states::ReactionType::Dislike)
    }
    pub fn reaction_remove(ctx: Context<RemoveReactionContext>) -> Result<()> {
        remove_reaction(ctx)
    }
    pub fn comment_tweet(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
        add_comment(ctx, comment_content)
    }
    pub fn comment_remove(ctx: Context<RemoveCommentContext>) -> Result<()> {
        remove_comment(ctx)
    }
}
