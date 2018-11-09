pragma solidity ^0.4.24;

import "../common/model_type.sol";
import "../common/admin.sol";
import "../common/address.sol";
import "../interfaces/sys_config.sol";

/// @title System config contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract SysConfig is ISysConfig, EconomicalType, ReservedAddress {

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
    uint chainIdV1;

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
        uint _chainId,
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
        chainId = uint32(_chainId);
        chainIdV1 = _chainId;
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

    function updateToChainIdV1()
        external
        onlyAdmin
    {
        chainIdV1 = uint(chainId);
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

    function getChainIdV1()
        public
        view
        returns (uint)
    {
        return chainIdV1;
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
