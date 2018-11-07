pragma solidity ^0.4.24;

import "../interfaces/IAuthorization.sol";
import "../interfaces/IAllGroups.sol";
import "../interfaces/IGroup.sol";
import "./ReservedAddrPublic.sol";

/// @title The modifier for checking permission
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
contract Check is ReservedAddrPublic {

    modifier hasPermission(address _permission) {
        require(
            checkPermissionWithGroup(msg.sender, _permission),
            "permission denied."
        );
        _;
    }

    function checkPermissionWithGroup(address _account, address _permission)
        private
        returns (bool)
    {
        IAuthorization auth = IAuthorization(authorizationAddr);
        if (!auth.checkPermission(msg.sender, _permission)) {
            IAllGroups groups = IAllGroups(allGroupsAddr);
            address[] memory allGroups = groups.queryGroups();
            IGroup group;
            for (uint i; i < allGroups.length; i++) {
                group = IGroup(allGroups[i]);
                if (group.inGroup(_account) &&
                    auth.checkPermission(allGroups[i], _permission))
                {
                    return true;
                }
            }
            return false;
        }
        return true;
    }
}
