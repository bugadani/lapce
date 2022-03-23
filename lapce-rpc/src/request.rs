use std::path::PathBuf;

use lsp_types::{CompletionItem, Position};
use serde::Serialize;

#[derive(Serialize)]
pub struct RpcRequestObject {
    pub id: u64,
    pub method: String,
    pub params: RpcRequestParams,
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
