use serde::{Deserialize, Serialize};
use std::error::Error;
use html_escape::decode_html_entities;

#[derive(Debug, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    systemInstruction: SystemInstruction,
    generationConfig: GenerationConfig,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

#[derive(Debug, Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Debug, Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Debug, Deserialize)]
struct ResponsePart {
    text: String,
}

#[derive(Debug, Deserialize)]
struct JsonResponse {
    html: String,
}

#[derive(Debug, Serialize)]
struct SystemInstruction {
    parts: Vec<Part>,
    role: String,
}

#[derive(Debug, Serialize)]
struct GenerationConfig {
    responseMimeType: String,
    responseSchema: ResponseSchema,
}

#[derive(Debug, Serialize)]
struct ResponseSchema {
    #[serde(rename = "type")]
    type_field: String,
    properties: SchemaProperties,
}

#[derive(Debug, Serialize)]
struct SchemaProperties {
    html: HtmlProperty,
}

#[derive(Debug, Serialize)]
struct HtmlProperty {
    #[serde(rename = "type")]
    type_field: String,
}

pub struct GeminiClient {
    api_key: String,
    client: reqwest::Client,
}

impl GeminiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn generate_text(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-pro-exp-03-25:generateContent?key={}",
            self.api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
            }],
            systemInstruction: SystemInstruction {
                parts: vec![Part {
                    text: "与えられたプロンプトの印象に添ったHTMLを、tailwindで装飾して返します。但し以下の点に注意してください：
1. コンテンツの内容は後述の文章をそのまま表示し、あくまで「HTML」と「tailwind」での装飾だけを変化させ返します。
2. JavaScriptのコードは一切使用しないでください。
3. 但し、HTMLの複雑な構造やCSSのアニメーションを多用し、できるだけダイナミックに変化させてください。
4. そのHTMLは html というフィールドに入れてください。
5. HTMLの属性値は必ずシングルクォート（'）で囲んでください。
6. クラス名は必ずシングルクォートで囲み、スペースで区切ってください。
7. エスケープ文字は使用せず、生のHTMLとして返してください。

コンテンツの内容の文章は以下のとおりです。「あのイーハトーヴォのすきとおった風、夏でも底に冷たさをもつ青いそら、うつくしい森で飾られたモリーオ市、郊外のぎらぎらひかる草の波。」".to_string(),
                }],
                role: "model".to_string(),
            },
            generationConfig: GenerationConfig {
                responseMimeType: "application/json".to_string(),
                responseSchema: ResponseSchema {
                    type_field: "object".to_string(),
                    properties: SchemaProperties {
                        html: HtmlProperty {
                            type_field: "string".to_string(),
                        },
                    },
                },
            },
        };

        let response = match self.client.post(&url).json(&request).send().await {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.text().await?;

                if !status.is_success() {
                    eprintln!(
                        "APIエラー: ステータスコード {} - レスポンス: {}",
                        status, body
                    );
                    return Err(format!("APIエラー: ステータスコード {}", status).into());
                }

                match serde_json::from_str::<GeminiResponse>(&body) {
                    Ok(gemini_resp) => {
                        let json_str = gemini_resp
                            .candidates
                            .first()
                            .and_then(|c| c.content.parts.first())
                            .map(|p| p.text.clone())
                            .ok_or_else(|| "レスポンスにテキストが含まれていません")?;

                        match serde_json::from_str::<JsonResponse>(&json_str) {
                            Ok(json_resp) => decode_html_entities(&json_resp.html).to_string(),
                            Err(e) => {
                                eprintln!("JSONのパース中にエラーが発生しました: {}", e);
                                eprintln!("JSON文字列: {}", json_str);
                                return Err(Box::new(e));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("レスポンスのパース中にエラーが発生しました: {}", e);
                        eprintln!("レスポンス本文: {}", body);
                        return Err(Box::new(e));
                    }
                }
            }
            Err(e) => {
                eprintln!("APIリクエスト中にエラーが発生しました: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(response)
    }
}
