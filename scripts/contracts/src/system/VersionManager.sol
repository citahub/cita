pragma solidity 0.4.24;

import "../common/Admin.sol";
import "../common/ReservedAddrPublic.sol";
import "../../interaction/interface/IVersionManager.sol";
import "../../interaction/interface/ISysConfig.sol";

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

    function setProtocolVersion(uint32 _version)
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

    /// @notice Deprecated. Check the setProtocolVersion
    function setVersion(uint32 _version)
        public
        onlyAdmin
    {
        setProtocolVersion(_version);
    }

    function getProtocolVersion()
        public
        view
        returns (uint32)
    {
        return version;
    }

    /// @notice Deprecated. Check the getProtocolVersion
    function getVersion()
        public
        view
        returns (uint32)
    {
        return getProtocolVersion();
    }
}
