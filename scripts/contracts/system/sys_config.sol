pragma solidity ^0.4.18;


/// @title The interface of system config
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface SysConfigInterface {
    /// @notice Update current chain name
    function setChainName(string) public;

    /// @notice Update current operator
    function setOperator(string) public;

    /// @notice Update current operator's website URL
    function setWebsite(string) public;

    /// @notice Get delay block number before validate
    function getDelayBlockNumber() public view returns (uint);

    /// @notice Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() public view returns (bool);

    /// @notice Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() public view returns (bool);

    /// @notice The name of current chain
    function getChainName() public view returns (string);

    /// @notice The id of current chain
    function getChainId() public view returns (uint32);

    /// @notice The operator of current chain
    function getOperator() public view returns (string);

    /// @notice Current operator's website URL
    function getWebsite() public view returns (string);

    /// @notice The interval time for creating a block (milliseconds)
    function getBlockInterval() public view returns (uint64);

    /// @notice The token information
    function getTokenInfo() public view returns (string, string, string);
}


/// @title System config contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract SysConfig is SysConfigInterface {

    /// @notice only chain_name, operator, website can be updated
    uint delayBlockNumber;
    bool checkPermission;
    bool checkQuota;
    string chainName;
    uint32 chainId;
    string operator;
    string website;
    uint64 blockInterval;
    EconomicalModel economicalModel;
    TokenInfo tokenInfo;

    enum EconomicalModel { Quota, Charge }

    struct TokenInfo {
        string name;
        string symbol;
        string avatar;
    }

    /// @notice Setup
    function SysConfig(
        uint _delayBlockNumber,
        bool _checkPermission,
        bool _checkQuota,
        string _chainName,
        uint32 _chainId,
        string _operator,
        string _website,
        uint64 _blockInterval,
        EconomicalModel _economicalModel,
        string _name,
        string _symbol,
        string _avatar
    )
        public
    {
        require(_chainId > 0);
        delayBlockNumber = _delayBlockNumber;
        checkPermission = _checkPermission;
        checkQuota = _checkQuota;
        chainName = _chainName;
        chainId = _chainId;
        operator = _operator;
        website = _website;
        blockInterval = _blockInterval;
        economicalModel = _economicalModel;
        tokenInfo = TokenInfo({
            name: _name,
            symbol: _symbol,
            avatar: _avatar
        });
    }

    function setOperator(string _operator)
        public
    {
        operator = _operator;
    }

    function setWebsite(string _website)
        public
    {
        website = _website;
    }

    function setChainName(string _chainName)
        public
    {
        chainName = _chainName;
    }

    function getDelayBlockNumber()
        public
        view
        returns (uint)
    {
        return delayBlockNumber;
    }

    function getPermissionCheck()
        public
        view
        returns (bool)
    {
        return checkPermission && (economicalModel == EconomicalModel.Quota);
    }

    function getQuotaCheck()
        public
        view
        returns (bool)
    {
        return checkQuota && (economicalModel == EconomicalModel.Quota);
    }

    function getChainName()
        public
        view
        returns (string)
    {
        return chainName;
    }

    function getChainId()
        public
        view
        returns (uint32)
    {
        return chainId;
    }

    function getOperator()
        public
        view
        returns (string)
    {
        return operator;
    }

    function getWebsite()
        public
        view
        returns (string)
    {
        return website;
    }

    function getBlockInterval()
        public
        view
        returns (uint64)
    {
        return blockInterval;
    }

    function getEconomicalModel()
        public
        view
        returns (EconomicalModel)
    {
        return economicalModel;
    }

    function getTokenInfo()
        public
        view
        returns(string name, string symbol, string avatar)
    {
        name = tokenInfo.name;
        symbol = tokenInfo.symbol;
        avatar = tokenInfo.avatar;
    }
}
