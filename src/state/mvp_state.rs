use serenity::model::id::UserId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MvpState {
    pub icon_url: Option<String>,
    pub mvp: Option<Mvp>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Mvp {
    pub id: UserId,
    pub icon_url: String
}