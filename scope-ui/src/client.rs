use std::collections::HashMap;

use anyhow::{bail, Context};

pub struct AppConfig {
    pub openflexure_url: url::Url,
    pub phoenix_url: url::Url,
}

pub struct OpenFlexurePosition {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl TryFrom<serde_json::Value> for OpenFlexurePosition {
    type Error = anyhow::Error;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        let pos = (value.get("x"), value.get("y"), value.get("z"));
        if let (Some(x), Some(y), Some(z)) = pos {
            if let (Some(x), Some(y), Some(z)) = (x.as_i64(), y.as_i64(), z.as_i64()) {
                return Ok(Self { x, y, z });
            }
        }

        bail!("Failed to parse openflexure positon from input")
    }
}

#[derive(Debug)]
pub enum OpenflexureAxis {
    X,
    Y,
    Z,
}

#[derive(Debug)]
pub enum MoveDirection {
    Pos(OpenflexureAxis, usize),
    Neg(OpenflexureAxis, usize),
}

pub struct AppClient {
    openflexure_url: url::Url,
    phoenix_url: url::Url,
}

impl AppClient {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            openflexure_url: config.openflexure_url.clone(),
            phoenix_url: config.openflexure_url.clone(),
        }
    }

    pub async fn get_openflexure_position(&self) -> anyhow::Result<OpenFlexurePosition> {
        let url = self
            .openflexure_url
            .join("api/v2/instrument/state/stage/position")?;

        let json: serde_json::Value = reqwest::Client::new()
            .get(url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .context("Failed to send request to get openflexure stage position")?
            .json()
            .await
            .context("Failed to parse response to json")?;

        json.try_into()
    }

    pub async fn openflexure_step(
        &self,
        direction: MoveDirection,
    ) -> anyhow::Result<reqwest::Response> {
        match direction {
            MoveDirection::Pos(axis, _) => self.move_axis(axis, 200),
            MoveDirection::Neg(axis, _) => self.move_axis(axis, -200),
        }
        .await
    }

    async fn move_axis(
        &self,
        axis: OpenflexureAxis,
        value: i64,
    ) -> anyhow::Result<reqwest::Response> {
        let current_pos = self.get_openflexure_position().await?; // TODO: not optimal

        let url = self.openflexure_url.join("api/v2/actions/stage/move")?;
        let mut body = HashMap::from(match axis {
            OpenflexureAxis::X => [
                ("x", (current_pos.x + value).to_string()),
                ("y", current_pos.y.to_string()),
                ("z", current_pos.z.to_string()),
            ],
            OpenflexureAxis::Y => [
                ("x", current_pos.x.to_string()),
                ("y", (current_pos.y + value).to_string()),
                ("z", current_pos.z.to_string()),
            ],
            OpenflexureAxis::Z => [
                ("x", current_pos.x.to_string()),
                ("y", current_pos.y.to_string()),
                ("z", (current_pos.z + value).to_string()),
            ],
        });

        body.insert("absolute", "true".to_string());

        reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await
            .context("Failed to post move axis request")
    }
}
