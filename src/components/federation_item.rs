use fedimint_core::config::FederationId;
use fedimint_core::Amount;
use iced::{
    widget::{row, text},
    Element,
};

use crate::Message;

#[derive(Debug, Clone)]
pub struct FederationItem {
    pub id: FederationId,
    pub name: String,
}

pub fn h_federation_item(item: &FederationItem, balance: Amount) -> Element<Message> {
    let FederationItem { id: _, name } = item;
    let row = row![
        text(name).size(24),
        // text(format!("{id}")).size(24),
        text(format!("{} sats", balance.sats_round_down())).size(24),
    ]
    .spacing(16);

    row.into()
}
