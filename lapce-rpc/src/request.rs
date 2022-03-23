use std::path::PathBuf;

use lsp_types::{CompletionItem, Position};
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(untagged)]
pub enum RpcRequestObject {
    Request {
        id: u64,
        method: String,
        params: RpcRequestParams,
    },
    Notification {
        method: String,
        params: Value,
    },
}

// TODO: move this into lapce-proxy
#[derive(Serialize)]
#[serde(untagged)]
pub enum RpcRequestParams {
    BufferHead {
        buffer_id: u64,
        path: PathBuf,
    },
    GlobalSearch {
        pattern: String,
    },
    NewBuffer {
        buffer_id: u64,
        path: PathBuf,
    },
    Save {
        rev: u64,
        buffer_id: u64,
    },
    GetCompletion {
        request_id: u64,
        buffer_id: u64,
        position: Position,
    },
    CompletionResolve {
        buffer_id: u64,
        completion_item: CompletionItem,
    },
    GetSignature {
        buffer_id: u64,
        position: Position,
    },
    GetReferences {
        buffer_id: u64,
        position: Position,
    },
    GetFiles {
        path: PathBuf,
    },
    ReadDir {
        path: PathBuf,
    },
    GetDefinition {
        request_id: u64,
        buffer_id: u64,
        position: Position,
    },
    GetDocumentSymbols {
        buffer_id: u64,
    },
    GetCodeActions {
        buffer_id: u64,
        position: Position,
    },
    GetDocumentFormatting {
        buffer_id: u64,
    },
}
