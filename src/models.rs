use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingRequest {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingResponse {
  pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TauriEventSse {
    id: String,
    type_: String,
    data: String,
}

impl TauriEventSse {
    pub fn from_client_event_sse(client_event_sse: sse_client::Event) -> Self {
      TauriEventSse { id: client_event_sse.id, type_: client_event_sse.type_, data: client_event_sse.data }
    }
}