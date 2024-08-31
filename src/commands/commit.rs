use anyhow::Context;
use std::path::Path;

use crate::commands;

pub(crate) fn invoke(message: String) -> anyhow::Result<()> {
    let head_ref = std::fs::read_to_string(".git/HEAD").context("read HEAD")?;
    let Some(head_ref) = head_ref.strip_prefix("ref: ") else {
        anyhow::bail!("refusing to commit onto detached HEAD");
    };
    let head_ref = head_ref.trim();
    let parent_hash = std::fs::read_to_string(format!(".git/{head_ref}"))
        .with_context(|| format!("read HEAD reference target '{head_ref}'"))?;
    let parent_hash = parent_hash.trim();

    let Some(tree_hash) =
        commands::write_tree::write_tree_for(Path::new(".")).context("write tree")?
    else {
        eprintln!("not committing empty tree");
        return Ok(());
    };

    let commit_hash =
        commands::commit_tree::write_commit(&message, &hex::encode(tree_hash), Some(parent_hash))
            .context("create commit")?;
    let commit_hash = hex::encode(commit_hash);

    std::fs::write(format!(".git/{head_ref}"), &commit_hash)
        .with_context(|| format!("update HEAD reference target {head_ref}"))?;

    println!("HEAD is now at {commit_hash}");

    Ok(())
}
