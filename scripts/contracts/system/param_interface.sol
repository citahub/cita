pragma solidity ^0.4.18;

interface ParamConstantInterface {

    /// Get the valid number in the system
    function getNumber() public view returns (uint);
    /// Whether check permission in the system or not, true represents check and false represents don't check.
    function getPermissionCheck() public view returns (bool);
    /// Whether check quota in the system or not, true represents check and false represents don't check.
    function getQuotaCheck() public view returns (bool);
}
