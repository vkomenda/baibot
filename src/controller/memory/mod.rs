use crate::{Bot, entity::MessageContext};
use matrix_sdk::ruma::events::room::message::FileMessageEventContent;
use mxlink::MatrixLink;

pub async fn handle_file(
    _bot: &Bot,
    matrix_link: MatrixLink,
    _message_context: &MessageContext,
    event_content: &FileMessageEventContent,
) -> anyhow::Result<()> {
    tracing::debug!("Handling a file");

    let media = matrix_link.client().media();
    let maybe_bytes = media.get_file(event_content, false).await?;

    if let Some(bytes) = maybe_bytes {
        // let temp_dir = std::env::temp_dir();
        let filename = event_content
            .filename
            .to_owned()
            .unwrap_or(event_content.body.to_owned());

        let n_bytes = bytes.len();
        tracing::debug!("Downloaded {filename} of size {n_bytes} bytes");
    } else {
        anyhow::bail!("No file content");
    };

    Ok(())
}
