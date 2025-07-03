//! MCP server implementation for Recovery Services tools

use anyhow::Result;
use mcp_types::JSONRPCMessage;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, info};

mod message_processor;

use message_processor::MessageProcessor;

/// Size of the bounded channels used to communicate between tasks
const CHANNEL_CAPACITY: usize = 128;

/// Run the MCP server using stdin/stdout
pub async fn run_server() -> Result<()> {
    info!("Starting Recovery Services MCP Server");
    
    // Set up channels for message passing
    let (incoming_tx, mut incoming_rx) = mpsc::channel::<JSONRPCMessage>(CHANNEL_CAPACITY);
    let (outgoing_tx, mut outgoing_rx) = mpsc::channel::<JSONRPCMessage>(CHANNEL_CAPACITY);

    // Task: read from stdin, push to incoming_tx
    let stdin_reader_handle = tokio::spawn({
        let incoming_tx = incoming_tx.clone();
        async move {
            let stdin = tokio::io::stdin();
            let reader = BufReader::new(stdin);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap_or_default() {
                match serde_json::from_str::<JSONRPCMessage>(&line) {
                    Ok(msg) => {
                        if incoming_tx.send(msg).await.is_err() {
                            break;
                        }
                    }
                    Err(e) => error!("Failed to deserialize JSONRPCMessage: {e}"),
                }
            }
            debug!("stdin reader finished (EOF)");
        }
    });

    // Task: process incoming messages
    let processor_handle = tokio::spawn({
        let mut processor = MessageProcessor::new(outgoing_tx.clone());
        async move {
            while let Some(msg) = incoming_rx.recv().await {
                processor.process_message(msg).await;
            }
            info!("processor task exited (channel closed)");
        }
    });

    // Task: write outgoing messages to stdout
    let stdout_writer_handle = tokio::spawn(async move {
        let mut stdout = tokio::io::stdout();
        while let Some(msg) = outgoing_rx.recv().await {
            match serde_json::to_string(&msg) {
                Ok(json) => {
                    if let Err(e) = stdout.write_all(json.as_bytes()).await {
                        error!("Failed to write to stdout: {e}");
                        break;
                    }
                    if let Err(e) = stdout.write_all(b"\n").await {
                        error!("Failed to write newline to stdout: {e}");
                        break;
                    }
                    if let Err(e) = stdout.flush().await {
                        error!("Failed to flush stdout: {e}");
                        break;
                    }
                }
                Err(e) => error!("Failed to serialize JSONRPCMessage: {e}"),
            }
        }
        info!("stdout writer exited (channel closed)");
    });

    // Wait for all tasks to finish
    let _ = tokio::join!(stdin_reader_handle, processor_handle, stdout_writer_handle);

    Ok(())
}