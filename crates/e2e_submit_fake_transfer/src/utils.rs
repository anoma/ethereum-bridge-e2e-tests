use colored::Colorize;
use eyre::{eyre, Result};
use std::{future::Future, time::Duration};
use tokio::time::timeout;

use self::step::StepResult;

pub mod step {
    #[derive(Debug, Clone)]
    pub enum Outcome {
        Succeeded,
        Skipped { reason: String },
        Failed { reason: String },
    }

    pub struct StepResult<T> {
        pub(super) outcome: Outcome,
        pub(super) debug_logs: Option<String>,
        pub(super) returning: Option<T>,
    }

    impl<T> StepResult<T> {
        pub fn succeeded() -> Self {
            Self {
                outcome: Outcome::Succeeded,
                debug_logs: None,
                returning: None,
            }
        }

        pub fn skipped(reason: impl AsRef<str>) -> Self {
            Self {
                outcome: Outcome::Skipped {
                    reason: reason.as_ref().to_owned(),
                },
                debug_logs: None,
                returning: None,
            }
        }

        pub fn failed(reason: impl AsRef<str>) -> Self {
            Self {
                outcome: Outcome::Failed {
                    reason: reason.as_ref().to_owned(),
                },
                debug_logs: None,
                returning: None,
            }
        }
    }

    impl<T> StepResult<T>
    where
        T: Clone,
    {
        pub fn with_debug_logs(&self, debug_logs: String) -> Self {
            StepResult {
                outcome: self.outcome.clone(),
                debug_logs: Some(debug_logs),
                returning: self.returning.clone(),
            }
        }

        pub fn returning(&self, returning: T) -> Self {
            StepResult {
                outcome: self.outcome.clone(),
                debug_logs: self.debug_logs.clone(),
                returning: Some(returning),
            }
        }
    }
}

/// The default timeout for any steps executed by a test runner. Steps can
/// take at most this amount of time to complete. If they would take longer,
/// they're aborted - this should catch when we have test steps which may
/// hang for whatever reason.
const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

#[derive(Debug)]
pub struct TestRunner {
    default_timeout: Duration,
    steps_taken: Vec<String>,
}

impl Default for TestRunner {
    fn default() -> Self {
        Self {
            default_timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECONDS),
            steps_taken: Default::default(),
        }
    }
}

impl TestRunner {
    pub fn log_info(&self, message: impl AsRef<str>) {
        println!("{}: {}", ">>> INFO".purple(), message.as_ref());
        println!();
    }

    pub async fn execute_step(
        &mut self,
        description: impl AsRef<str>,
        future: impl Future<Output = Result<StepResult<()>>>,
    ) -> Result<()> {
        self.execute_step_and_return(description, future)
            .await
            .map(|_| ())
    }

    pub async fn execute_step_and_return<T>(
        &mut self,
        description: impl AsRef<str>,
        future: impl Future<Output = Result<StepResult<T>>>,
    ) -> Result<Option<T>> {
        println!("{}: {}", ">>> ACTION".bold().blue(), description.as_ref());
        self.steps_taken.push(description.as_ref().to_string());
        println!();
        let step_result = match timeout(self.default_timeout, future).await {
            Ok(result) => match result {
                Ok(step_result) => match step_result.outcome {
                    step::Outcome::Succeeded => {
                        println!("{}", ">>> OUTCOME: succeeded".green());
                        println!();
                        if let Some(ref debug_logs) = step_result.debug_logs {
                            println!("debug logs: {}", debug_logs);
                            println!();
                        }
                        step_result
                    }
                    step::Outcome::Skipped { ref reason } => {
                        println!("{}: {reason}", ">>> OUTCOME: skipped".yellow());
                        println!();
                        if let Some(ref debug_logs) = step_result.debug_logs {
                            println!("debug logs: {}", debug_logs);
                            println!();
                        }
                        step_result
                    }
                    step::Outcome::Failed { reason } => {
                        println!("{}: {reason}", ">>> OUTCOME: failed".bright_blue());
                        println!();
                        if let Some(debug_logs) = step_result.debug_logs {
                            println!("debug logs: {}", debug_logs);
                            println!();
                        }
                        return Err(eyre!("Test failed"));
                    }
                },
                Err(error) => {
                    println!("{}", ">>> OUTCOME: errored".bright_red());
                    println!();
                    return Err(error);
                }
            },
            Err(timeout) => {
                println!("{}", ">>> OUTCOME: timed out".bright_red());
                println!();
                return Err(eyre!("Timed out: {:#?}", timeout));
            }
        };
        println!("###");
        println!();
        Ok(step_result.returning)
    }
}
