use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct LLMClient {
    client: Client,
    api_key: String,
    base_url: String,
    messages: Vec<Message>,
}

/// 数据结构
#[derive(Serialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
}

#[derive(Deserialize)]
struct ResponseBody {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

impl LLMClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
            messages: vec![Message {
                role: "system".to_string(),
                content: "你是一个智能助手".to_string(),
            }],
        }
    }

    pub async fn chat(&mut self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.messages.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let body = RequestBody {
            model: "my_model".to_string(),
            messages: self.messages.clone(),
        };

        let res = self
            .client
            .post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let json: ResponseBody = res.json().await?;

        if json.choices.is_empty() {
            return Ok("LLM 没有返回内容".to_string());
        }

        let reply = json.choices[0].message.content.clone();

        self.messages.push(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(reply)
    }

    pub async fn chat_stream(&mut self, prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
        let body = serde_json::json!({
            "model": "my_model",
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "stream": true   // 👈 开启流式
        });

        let res = self
            .client
            .post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        // 👉 获取字节流
        let mut stream = res.bytes_stream();
        let mut reply = String::new();
        while let Some(item) = stream.next().await {
            let chunk = item?;

            // 👉 转成字符串
            let text = String::from_utf8_lossy(&chunk);

            // 👉 SSE 格式：data: {...}
            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];

                    if data == "[DONE]" {
                        println!();
                        return Ok(());
                    }

                    // 👉 解析 JSON
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            // 👉 实时打印（关键）
                            print!("{}", content);
                            reply.push_str(content);
                            std::io::Write::flush(&mut std::io::stdout()).unwrap();
                        }
                    }
                }
            }
        }

        self.messages.push(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(())
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
