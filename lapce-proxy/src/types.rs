use std::collections::HashMap;
use std::path::PathBuf;

use crate::buffer::BufferId;
use crate::dispatch::DiffInfo;
use crate::dispatch::FileNodeItem;
use crate::plugin::PluginDescription;
use crate::terminal::TermId;
use anyhow::anyhow;
use anyhow::Result;
use lapce_core::style::LineStyle;
use lapce_rpc::Call;
use lapce_rpc::RpcMessage;
use lsp_types::ProgressParams;
use lsp_types::PublishDiagnosticsParams;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "method", content = "params")]
pub enum AppNotification {
    ProxyConnected {},
    SemanticStyles {
        rev: u64,
        buffer_id: BufferId,
        path: PathBuf,
        len: usize,
        styles: Vec<LineStyle>,
    },
    ReloadBuffer {
        buffer_id: BufferId,
        new_content: String,
        rev: u64,
    },
    PublishDiagnostics {
        diagnostics: PublishDiagnosticsParams,
    },
    WorkDoneProgress {
        progress: ProgressParams,
    },
    HomeDir {
        path: PathBuf,
    },
    InstalledPlugins {
        plugins: HashMap<String, PluginDescription>,
    },
    ListDir {
        items: Vec<FileNodeItem>,
    },
    DiffFiles {
        files: Vec<PathBuf>,
    },
    DiffInfo {
        diff: DiffInfo,
    },
    UpdateTerminal {
        term_id: TermId,
        content: String,
    },
    CloseTerminal {
        term_id: TermId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppMessage {
    Notification(AppNotification),
    // TODO: split this into Success and Error
    Response(Value),
    Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppRequest {}

impl RpcMessage for AppMessage {
    type Notification = AppNotification;
    type Request = AppRequest;

    fn is_response(&self) -> bool {
        matches!(self, Self::Response(_))
    }

    fn get_id(&self) -> Option<u64> {
        match self {
            AppMessage::Notification(_) => None,
            AppMessage::Response(rsp) => rsp.as_object().unwrap()["id"].as_u64(),
            AppMessage::Request => None,
        }
    }

    fn into_response(self) -> Result<Result<Value, Value>, String> {
        if let AppMessage::Response(mut resp) = self {
            // TODO this should be split up into a success and a response type, so this check
            // will be made meaningless
            if resp.get("result").is_some() == resp.get("error").is_some() {
                return Err("RPC response must contain exactly one of\
                            'error' or 'result' fields."
                    .into());
            }
            let result = resp.as_object_mut().and_then(|obj| obj.remove("result"));
            match result {
                Some(resp) => Ok(Ok(resp)),
                None => {
                    let error = resp
                        .as_object_mut()
                        .and_then(|obj| obj.remove("error"))
                        .unwrap();
                    Ok(Err(error))
                }
            }
        } else {
            todo!()
        }
    }

    fn into_rpc(self) -> Result<Call<AppNotification, AppRequest>> {
        match self {
            AppMessage::Notification(not) => Ok(Call::Notification(not)),
            AppMessage::Response(_) => Err(anyhow!("")),
            AppMessage::Request => unimplemented!(),
        }
    }
}
