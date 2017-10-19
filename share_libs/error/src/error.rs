//-32003             查询类错误
//-32006             交易认证类错误
//-32099             请求超时
pub enum ErrorCode {
    QueryError,
    TxAuthError,
    TimeOut,
}

impl ErrorCode {
    pub fn query_error() -> i64 {
        -32003
    }

    pub fn tx_auth_error() -> i64 {
        -32006
    }

    pub fn time_out_error() -> i64 {
        -32099
    }
}
