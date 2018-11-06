pragma solidity ^0.4.24;

import "../common/Admin.sol";
import "../common/ReservedAddrPublic.sol";
import "../interfaces/IVersionManager.sol";
import "../interfaces/ISysConfig.sol";

contract VersionManager is IVersionManager, ReservedAddrPublic {
    uint32 public version;

    Admin admin = Admin(adminAddr);

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    /// @notice Setup
    constructor(uint32 _version)
        public
    {
        version = _version;
    }

    function setVersion(uint32 _version)
        public
        onlyAdmin
    {
        if (_version != version + 1) {
            revert("New version must be greater by 1 than the older one.");
        }
        if (version == 0 && _version == 1) {
            ISysConfig config = ISysConfig(sysConfigAddr);
            config.updateToChainIdV1();
        }
        version = _version;
    }

    function getVersion()
        public
        view
        returns (uint32)
    {
        return version;
    }
}
