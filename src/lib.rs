mod utils;

use futures_util::io::AsyncBufReadExt;
use futures_util::io::AsyncWriteExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use websocket_async_io::WebsocketIO;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(start)]
pub fn start()->Result<(),JsValue> {
    spawn_local(async move{
        run().await.unwrap();
    });
    Ok(())
}

async fn run()->anyhow::Result<()>{
    let ws = WebsocketIO::new("localhost:8000").await?;
    let (mut reader, mut writer) = ws.split();
    writer.write_all(&[0, 1, 2, 3, 93]).await?;
    writer.write_all(&[42, 34, 93]).await?;
    writer.write_all(&[0, 0, 1, 2, 93]).await?;

    let mut buf = Vec::new();
    for _ in 0..3 {
        reader.read_until(93, &mut buf).await?;
        console_log!("{:?}", buf);
        buf.clear();
    }

    Ok(())
}