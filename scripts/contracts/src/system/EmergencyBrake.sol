pragma solidity ^0.4.24;

import "../common/Admin.sol";
import "../common/ReservedAddrPublic.sol";
import "../interfaces/IEmergencyBrake.sol";

contract EmergencyBrake is IEmergencyBrake, ReservedAddrPublic {
    bool public state;

    Admin admin = Admin(adminAddr);

    modifier onlyAdmin {
        if (admin.isAdmin(msg.sender))
            _;
        else return;
    }

    function setState(bool _state)
        public
        onlyAdmin
    {
        state = _state;
    }
}
