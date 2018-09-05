pragma solidity ^0.4.24;

import "../common/model_type.sol";
import "../common/admin.sol";
import "../common/address.sol";


/// @title The interface of system config
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
interface SysConfigInterface {
    /// @notice Update current chain name
    function setChainName(string) external;

    /// @notice Update current operator
    function setOperator(string) external;

    /// @notice Update current operator's website URL
    function setWebsite(string) external;

    /// @notice Get delay block number before validate
    function getDelayBlockNumber() external view returns (uint);

    /// @notice Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() external view returns (bool);

    /// @notice Check sender's send transaction permission
    function getSendTxPermissionCheck() external view returns (bool);

    /// @notice Check sender's create contract permission
    function getCreateContractPermissionCheck() external view returns (bool);

    /// @notice Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() external view returns (bool);

    /// @notice Whether check transaction fee back to operation platform or not, true represents back to platform and false represents back to nodes
    function getFeeBackPlatformCheck() external view returns (bool);

    /// @notice The owner of the chain
    function getChainOwner() external view returns (address);

    /// @notice The name of current chain
    function getChainName() external view returns (string);

    /// @notice The id of current chain
    function getChainId() external view returns (uint32);

    /// @notice The operator of current chain
    function getOperator() external view returns (string);

    /// @notice Current operator's website URL
    function getWebsite() external view returns (string);

    /// @notice The interval time for creating a block (milliseconds)
    function getBlockInterval() external view returns (uint64);

    /// @notice The token information
    function getTokenInfo() external view returns (string, string, string);
}


/// @title System config contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract SysConfig is SysConfigInterface, EconomicalType, ReservedAddress {

    /// @notice only chain_name, operator, website can be updated
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

    Admin admin = Admin(adminAddr);

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    struct TokenInfo {
        string name;
        string symbol;
        string avatar;
    }

    /// @notice Setup
    constructor(
        uint _delayBlockNumber,
        bool _checkPermission,
        bool _checkSendTxPermission,
        bool _checkCreateContractPermission,
        bool _checkQuota,
        bool _checkFeeBackPlatform,
        address _chainOwner,
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
        require(_chainId > 0, "The chainId should larger than zero.");
        delayBlockNumber = _delayBlockNumber;
        checkPermission = _checkPermission;
        checkSendTxPermission = _checkSendTxPermission;
        checkCreateContractPermission = _checkCreateContractPermission;
        checkQuota = _checkQuota;
        checkFeeBackPlatform = _checkFeeBackPlatform;
        chainOwner = _chainOwner;
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
        external
        onlyAdmin
    {
        operator = _operator;
    }

    function setWebsite(string _website)
        external
        onlyAdmin
    {
        website = _website;
    }

    function setChainName(string _chainName)
        external
        onlyAdmin
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
        return checkPermission;
    }

    function getSendTxPermissionCheck()
        public
        view
        returns (bool)
    {
        return checkSendTxPermission;
    }

    function getCreateContractPermissionCheck()
        public
        view
        returns (bool)
    {
        return checkCreateContractPermission;
    }

    function getQuotaCheck()
        public
        view
        returns (bool)
    {
        return checkQuota && (economicalModel == EconomicalModel.Quota);
    }

    function getFeeBackPlatformCheck()
        public
        view
        returns (bool)
    {
        return checkFeeBackPlatform;
    }

    function getChainOwner()
        public
        view
        returns (address)
    {
        return chainOwner;
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
