pub struct Sysconfig {
    uint delayBlockNumber;
    bool checkPermission;
    bool checkSendTxPermission;
    bool checkCreateContractPermission;
    bool checkQuota;
    bool checkFeeBackPlatform;
    address chainOwner;
    string chainName;
    uint32 chainId;
    string operator;
    string website;
    uint64 blockInterval;
    EconomicalModel economicalModel;
    TokenInfo tokenInfo;
    uint chainIdV1;
    bool autoExec;
}

    struct TokenInfo {
        string name;
        string symbol;
        string avatar;
    }

impl Contract for Sysconfig {
    fn create() {

    }

    fn execute() {
         method_tools::extract_to_u32(&params.data[..]).and_then(|signature| match signature {
            0 => self.init(params, data_provider),
            // Register function
            0x832b_4580 => self.balance_get(params, data_provider),
            0xaa91_543e => self.update(params, data_provider),
            _ => Err(NativeError::Internal("out of gas".to_string())),
    }
}

impl Sysconfig {
    fn init() {}

    fn setOperator() {}
}
