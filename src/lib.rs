#![allow(unused)]

use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, MessageEvent, WebSocket};
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
struct SyncMessage {
    key: String,
    value: String,
}

#[derive(Default)]
#[wasm_bindgen]
pub struct MemoryCache {
    cache: Arc<Mutex<BTreeMap<String, String>>>,
}

#[wasm_bindgen]
impl MemoryCache {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MemoryCache {
        console_error_panic_hook::set_once();

        MemoryCache {
            cache: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
    
    pub fn get_all(&self) -> String {
        let cache = self.cache.lock().unwrap();
        serde_json::to_string(&*cache).unwrap()
    }

    pub fn insert(&self, key: String, value: String) {
        let mut cache = self.cache.lock().unwrap();
        cache.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
        let cache = self.cache.lock().unwrap();
        cache.get(&key).cloned()
    }

    pub fn remove(&self, key: String) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(&key)
    }

    // Sync with remote engine
    pub async fn sync_with_remote(&self, url: String) -> Result<(), JsValue> {
        let ws = WebSocket::new(&url)?;

        let cache = self.cache.lock().unwrap();
        for (key, value) in cache.iter() {
            let message = SyncMessage {
                key: key.clone(),
                value: value.clone(),
            };

            let message_json = serde_json::to_string(&message).unwrap();
            let message_bytes = message_json.into_bytes();
            let uint8_array = Uint8Array::new_with_length(message_bytes.len() as u32);
            uint8_array.copy_from(&message_bytes);

            let blob = Blob::new_with_u8_array_sequence(&uint8_array)?;

            ws.send_with_blob(&blob)?;
        }

        let on_message = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(blob) = e.data().dyn_into::<Blob>() {
                wasm_bindgen_futures::spawn_local(async move {
                    let array = JsFuture::from(blob.array_buffer()).await.unwrap();
                    let uint8_array = Uint8Array::new(&array);
                    let bytes = uint8_array.to_vec();
                    let message_json = String::from_utf8(bytes).unwrap();
                    let message: SyncMessage = serde_json::from_str(&message_json).unwrap();

                    // Handle incoming sync message
                    println!("{:#?}", message);
                });
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
        on_message.forget();

        Ok(())
    }
    
    // Fetch from remote engine
    // pub async fn fetch_from_remote(&self, url: String) -> Result<(), JsValue> {
    //     let request = RequestInit::new();
    //     request.open("GET", &url).unwrap();
    //     request.set_response_type(XmlHttpRequestResponseType::Json);
    //     request.send().unwrap();

    //     let response = JsFuture::from(request.clone()).await?;
    //     if request.status() == 200 {
    //         let json = response.as_string().unwrap();
    //         let data: Vec<SyncMessage> = serde_json::from_str(&json).unwrap();

    //         let mut cache = self.cache.lock().unwrap();
    //         for message in data {
    //             cache.insert(message.key, message.value);
    //         }

    //         Ok(())
    //     } else {
    //         Err(JsValue::from_str("Failed to fetch data"))
    //     }
    // }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
