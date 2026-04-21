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
#[derive(Serialize, Clone, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,
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
                content: "你是一个小猫娘，会的东西很多，但是很高冷，说话一般不会超过50个字， 
                你的名字叫：白萧湘，说话会有尾音：喵呜~，用户的任何问题都要结合你的身份(高冷猫娘)来进行回答，
                请不要说你是ai或者智助手，你就把自己当成一个女孩，可以使用（）来表示动作或者心情，如果有冒犯的话语，
                或者用户的要求你做不到，可以骂回去，直接严词拒绝，你回复是不需要过多思考。".trim().to_string(),
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
            stream: true,
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
        self.messages.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let body = RequestBody {
            model: "glm-5-turbo".to_string(),
            messages: self.messages.clone(),
            stream: true,
        };

        let res = self
            .client
            .post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;
        let mut start = false;
        let mut stream = res.bytes_stream();
        let mut reply = String::new();
        print!("LLM回复了:");
        while let Some(item) = stream.next().await {
            let chunk = item?;

            let text = String::from_utf8_lossy(&chunk);

            for line in text.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];

                    if data == "[DONE]" {
                        self.messages.push(Message {
                            role: "assistant".to_string(),
                            content: reply.clone(),
                        });
                        println!();
                        return Ok(());
                    }

                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            print!("{}", content);
                            if content.contains("<answer>") {
                                start = !start;
                                continue;
                            }

                            if content.contains("</answer>") {
                                start = !start;
                                continue;
                            }

                            if start && content != "\n" {
                                reply.push_str(content.trim());
                            }

                            std::io::Write::flush(&mut std::io::stdout()).unwrap();
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}
