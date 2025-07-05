use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum MarkstError {
    #[error("I/O Error: {0}")]
    Io(#[from] io::Error),

    #[error("Conversion failed: {0}")]
    Conversion(String),
    
    // 我们未来可以添加更多具体的错误类型，比如：
    // #[error("Markdown parsing failed")]
    // MarkdownParse,
}

// 这是一个别名，方便我们在库的其他地方使用
pub type Result<T> = std::result::Result<T, MarkstError>;