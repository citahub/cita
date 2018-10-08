pragma solidity ^0.4.24;

import "../permission_management/authorization.sol";
import "../user_management/all_groups.sol";
import "../user_management/group.sol";

/// @title The modifier for checking permission
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract Check is ReservedAddress {

    Authorization auth = Authorization(authorizationAddr);
    AllGroups groups = AllGroups(allGroupsAddr);

    modifier checkPermission(address _permission) {
        require(checkPermissionWithGroup(msg.sender, _permission), "permission denied.");
        _;
    }

    function checkPermissionWithGroup(address _account, address _permission)
        private
        view
        returns (bool)
    {
        if (!auth.checkPermission(msg.sender, _permission)) {
            address[] memory allGroups = groups.queryGroups();
            Group group;
            for (uint i; i< allGroups.length; i++) {
                group = Group(allGroups[i]);
                if (group.inGroup(_account) && auth.checkPermission(allGroups[i], _permission))
                    return true;
            }
            return false;
        }
        return true;
    }
}
