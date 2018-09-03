pragma solidity ^0.4.24;

import "../common/admin.sol";
import "../common/address.sol";

contract EmergencyBrake is ReservedAddress {
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
