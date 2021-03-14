use serde::{ Serialize, Deserialize };

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag="type", content="payload", rename_all="camelCase")]
pub enum Input {
    #[serde(rename="join")]
    Join(JoinInput)
    #[serde(rename="post")]
    Post(PostInput)
}

pub struct JoinInput {

}

pub enum Output {

}

pub enum OutputError {

}

pub struct UserPostedOutput {
    pub message: Message;
}