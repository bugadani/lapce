pub mod buffer;
pub mod dispatch;
pub mod lsp;
pub mod plugin;
pub mod terminal;
pub mod types;

use dispatch::Dispatcher;

pub fn mainloop() {
    println!("foo");
    let (sender, receiver) = lapce_rpc::stdio();
    let dispatcher = Dispatcher::new(sender);
    let _ = dispatcher.mainloop(receiver);
}
