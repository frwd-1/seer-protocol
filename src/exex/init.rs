use eyre::Result;
use futures::Future;
use reth::api::FullNodeComponents;
use reth_exex::ExExContext;

pub async fn exex_init<Node: FullNodeComponents>(
    ctx: ExExContext<Node>,
) -> Result<impl Future<Output = Result<()>>> {
    Ok(super::exex(ctx))
}
