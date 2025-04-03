use matrix_sdk::ruma::events::room::message::FileMessageEventContent;

#[derive(Debug)]
pub enum ControllerType {
    // Denotes that the message is to be ignored.
    Ignore,

    Help,

    UsageHelp,

    Unknown,

    Error(String),
    ErrorInThread(String, mxlink::ThreadInfo),

    ProviderHelp,

    Access(super::access::AccessControllerType),

    Agent(super::agent::AgentControllerType),

    Config(super::cfg::ConfigControllerType),

    ChatCompletion(super::chat_completion::ChatCompletionControllerType),

    ImageGeneration(String),
    StickerGeneration(String),

    // Upsert a file into a vector database
    Upsert(FileMessageEventContent),
}
