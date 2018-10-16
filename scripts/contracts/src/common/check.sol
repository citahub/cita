pragma solidity ^0.4.24;

import "../interfaces/authorization.sol";
import "../interfaces/all_groups.sol";
import "../interfaces/group.sol";
import "./address.sol";

/// @title The modifier for checking permission
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract Check is ReservedAddress {

    IAuthorization auth = IAuthorization(authorizationAddr);
    IAllGroups groups = IAllGroups(allGroupsAddr);

    modifier checkPermission(address _permission) {
        require(checkPermissionWithGroup(msg.sender, _permission), "permission denied.");
        _;
    }

    function checkPermissionWithGroup(address _account, address _permission)
        private
        returns (bool)
    {
        if (!auth.checkPermission(msg.sender, _permission)) {
            address[] memory allGroups = groups.queryGroups();
            IGroup group;
            for (uint i; i < allGroups.length; i++) {
                group = IGroup(allGroups[i]);
                if (group.inGroup(_account) && auth.checkPermission(allGroups[i], _permission))
                    return true;
            }
            return false;
        }
        return true;
    }
}
