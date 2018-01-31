pragma solidity ^0.4.18;

import "./param_interface.sol";

contract ParamConstant is ParamConstantInterface {

    uint valid_number;
    bool check_permission;
    bool check_quota;

    /// Setup
    function ParamConstant(uint _num, bool _perm, bool _quota) public {
        valid_number = _num;
        check_permission = _perm;
        check_quota = _quota;
    }

    function getNumber() public view returns (uint) {
        return valid_number;
    }

    function getPermissionCheck() public view returns (bool) {
        return check_permission;
    }

    function getQuotaCheck() public view returns (bool) {
        return check_quota;
    }
}
