use std::fs::read_to_string;
use std::io::Write;

pub type BoxResult<T> = Result<T, Box<dyn std::error::Error>>;

const STEPS_ENV_NAME: &str = "PIPEDREAM_STEPS";
const EXPORTS_ENV_NAME: &str = "PIPEDREAM_EXPORTS";

lazy_static::lazy_static! {
    pub static ref STEPS: serde_json::Value = {
        read_steps().unwrap_or(serde_json::Value::Null)
    };
}

fn read_steps() -> BoxResult<serde_json::Value> {
    let steps_file_name = std::env::var(STEPS_ENV_NAME)?;
    let null = "null".to_owned();
    let contents = read_to_string(steps_file_name).unwrap_or(null);

    Ok(serde_json::from_str(&contents)?)
}

pub fn export(name: &str, value: serde_json::Value) -> BoxResult<()> {
    let export_file_name = std::env::var(EXPORTS_ENV_NAME)?;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(export_file_name)?;

    writeln!(file, "{}:json={}", name, serde_json::to_string(&value)?,)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{export, read_steps, BoxResult, EXPORTS_ENV_NAME, STEPS_ENV_NAME};

    #[test]
    pub fn steps() -> BoxResult<()> {
        std::env::set_var(STEPS_ENV_NAME, "foo");
        assert!(read_steps()?.is_null());

        std::env::set_var(STEPS_ENV_NAME, "test-step-data.json");
        assert_eq!(
            serde_json::to_string(&read_steps()?)?,
            "{\"step1\":{\"data\":\"foo\"}}"
        );

        Ok(())
    }

    #[test]
    pub fn exports() -> BoxResult<()> {
        const FILE_NAME: &str = "test-exports";

        // This guard will remove the temporary file even in case of error
        struct Guard {}
        impl Drop for Guard {
            fn drop(&mut self) {
                std::fs::remove_file(std::path::Path::new(FILE_NAME)).ok();
            }
        }
        let _guard = Guard {};

        std::env::set_var(EXPORTS_ENV_NAME, FILE_NAME);

        export("test", "value".into())?;

        assert_eq!(
            std::fs::read_to_string(FILE_NAME)?.trim(),
            "test:json=\"value\""
        );

        Ok(())
    }
}
