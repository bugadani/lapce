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
    Success { id: u64, result: Value },
    Error { id: u64, error: Value }, // TODO: use a strongly typed Error struct
    Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppRequest {}

impl RpcMessage for AppMessage {
    type Notification = AppNotification;
    type Request = AppRequest;

    fn is_response(&self) -> bool {
        matches!(self, Self::Success { .. } | Self::Error { .. })
    }

    fn get_id(&self) -> Option<u64> {
        if let AppMessage::Success { id, .. } | AppMessage::Error { id, .. } = self {
            Some(*id)
        } else {
            None
        }
    }

    fn into_response(self) -> Result<Result<Value, Value>, String> {
        match self {
            AppMessage::Success { result, .. } => Ok(Ok(result)),
            AppMessage::Error { error, .. } => Ok(Err(error)),
            _ => Err(String::new()),
        }
    }

    fn into_rpc(self) -> Result<Call<AppNotification, AppRequest>> {
        match self {
            AppMessage::Notification(not) => Ok(Call::Notification(not)),
            AppMessage::Request => unimplemented!(),
            _ => Err(anyhow!("")),
        }
    }
}
