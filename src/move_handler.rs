use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum  InvalidMoveError {

}

pub fn move_unchecked() {

}

pub fn default_move_handler() -> Result<(), InvalidMoveError> {
    Ok(())
}
