
use crate::{state::ChatState, traits::api_client::ApiRequest};







pub struct App {
    pub api_client: Box<dyn ApiRequest>,
    pub chat_state: ChatState,
}

impl App {
    pub fn new(api_client: Box<dyn ApiRequest>, chat_state: ChatState) -> Self {
        App {
            api_client,
            chat_state,
        }
    }
}
