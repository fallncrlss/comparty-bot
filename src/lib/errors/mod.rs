#[derive(thiserror::Error, Debug)]
pub enum CacheError {
    #[error("{0}")]
    Execute(#[source] anyhow::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum DBError {
    #[error("{0}")]
    Execute(#[source] anyhow::Error),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum ChatError {
    #[error("{0}")]
    Insert(#[source] anyhow::Error),
    #[error("{0}")]
    GetSettings(#[source] anyhow::Error),
    #[error("{0}")]
    InsertSettings(#[source] anyhow::Error),
    #[error("{0}")]
    ChangeSettings(#[source] anyhow::Error),
    #[error("{0}")]
    MigrateChat(#[source] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AdminCommandsError {
    #[error("{0}")]
    GetRestrictMentions(#[source] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum AdminCommandsControllerError {
    #[error("{0}")]
    Report(#[source] anyhow::Error),
    #[error("{0}")]
    GetInfo(#[source] anyhow::Error),
    #[error("{0}")]
    MuteUser(#[source] anyhow::Error),
    #[error("{0}")]
    BanUser(#[source] anyhow::Error),
    #[error("{0}")]
    ChangeSettings(#[source] anyhow::Error),
    #[error("{0}")]
    GetSettings(#[source] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum MessageError {
    #[error("{0}")]
    GetCASStatus(#[source] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum MessageControllerError {
    #[error("{0}")]
    CheckLinkInMessage(#[source] anyhow::Error),
    #[error("{0}")]
    CheckNewMember(#[source] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum UserError {
    #[error("{0}")]
    Insert(#[source] anyhow::Error),
    #[error("{0}")]
    RepeatingRequestDuringCooldown(String),
    #[error("{0}")]
    GetRating(#[source] anyhow::Error),
    #[error("{0}")]
    GetInfo(#[source] anyhow::Error),
    #[error("{0}")]
    FetchRatingTop(#[source] anyhow::Error),
    #[error("{0}")]
    InsertRating(#[source] anyhow::Error),
    #[error("{0}")]
    DeleteRating(#[source] anyhow::Error),
    #[error("{0}")]
    Validation(#[source] anyhow::Error)
}
