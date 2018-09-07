pragma solidity ^0.4.24;

import "../common/admin.sol";
import "../common/address.sol";

contract VersionManager is ReservedAddress {
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
