use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum MailError {
  /// Invalid Instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
  /// Account Is Not Writable
  #[error("Account Is Not Writable")]
  NotWritable,
}

impl From<MailError> for ProgramError {
  fn from(e: MailError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
