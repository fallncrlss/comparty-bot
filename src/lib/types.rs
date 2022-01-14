use teloxide;

pub type ConfBot = teloxide::prelude::AutoSend<teloxide::adaptors::DefaultParseMode<teloxide::Bot>>;
pub type MessageContext = teloxide::prelude::UpdateWithCx<ConfBot, teloxide::prelude::Message>;
pub type CallbackContext =
    teloxide::prelude::UpdateWithCx<ConfBot, teloxide::prelude::CallbackQuery>;
