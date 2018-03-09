pragma solidity ^0.4.18;

interface ConstantConfigInterface {
    /// Get the valid number in the system
    function getNumber() public view returns (uint);
    /// Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() public view returns (bool);
    /// Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() public view returns (bool);
}

contract ConstantConfig is ConstantConfigInterface {

    uint valid_number;
    bool check_permission;
    bool check_quota;

    /// Setup
    function ConstantConfig(uint _num, bool _perm, bool _quota) public {
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
