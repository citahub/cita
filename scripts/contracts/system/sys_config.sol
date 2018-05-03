pragma solidity ^0.4.18;


/// @title The interface of system config
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface SysConfigInterface {
    /// @notice Get delay block number before validate
    function getDelayBlockNumber() public view returns (uint);

    /// @notice Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() public view returns (bool);

    /// @notice Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() public view returns (bool);

    /// @notice The name of current chain
    function getChainName() public view returns (string);
    /// @notice Update current chain name
    function setChainName(string) public;

    /// @notice The id of current chain
    function getChainId() public view returns (uint32);

    /// @notice The operator of current chain
    function getOperator() public view returns (string);
    /// @notice Update current operator
    function setOperator(string) public;

    /// @notice Current operator's website URL
    function getWebsite() public view returns (string);
    /// @notice Update current operator's website URL
    function setWebsite(string) public;

    /// @notice The interval time for creating a block (milliseconds)
    function getBlockInterval() public view returns (uint);
}


/// @title System config contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract SysConfig is SysConfigInterface {

    enum TransactionEconomicalModel { Quota, GasPrice }
    
    /// @notice only chain_name, operator, website can be updated
    uint delay_block_number;
    bool check_permission;
    bool check_quota;
    string chain_name;
    uint32 chain_id;
    string operator;
    string website;
    uint block_interval;
    TransactionEconomicalModel transactionEconomicalModel;

    /// @notice Setup
    function SysConfig(
        uint _delayBlockNumber,
        bool _checkPermission,
        bool _checkQuota,
        string _chainName,
        uint32 _chainId,
        string _operator,
        string _websiteUrl,
        uint _blockInterval,
        TransactionEconomicalModel _tx
    )
        public
    {
        delay_block_number = _delayBlockNumber;
        check_permission = _checkPermission;
        check_quota = _checkQuota;
        chain_name = _chainName;
        chain_id = _chainId;
        operator = _operator;
        website = _websiteUrl;
        block_interval = _blockInterval;
        transactionEconomicalModel = _tx;
    }

    function getDelayBlockNumber() public view returns (uint) {
        return delay_block_number;
    }

    function getPermissionCheck() public view returns (bool) {
        return check_permission;
    }

    function getQuotaCheck() public view returns (bool) {
        return check_quota;
    }

    function getChainName() public view returns (string) {
        return chain_name;
    }

    function getChainId() public view returns (uint32) {
        return chain_id;
    }

    function getOperator() public view returns (string) {
        return operator;
    }

    function getWebsite() public view returns (string) {
        return website;
    }

    function getBlockInterval() public view returns (uint) {
        return block_interval;
    }

    function getTransactionEconomicalModel() public view returns (TransactionEconomicalModel) {
        return transactionEconomicalModel;
    }

    function setOperator(string _operator) public {
        operator = _operator;
    }

    function setWebsite(string _websiteUrl) public {
        website = _websiteUrl;
    }

    function setChainName(string _chainName) public {
        chain_name = _chainName;
    }
}
