pragma solidity ^0.4.24;

/* solium-disable */

/// @title The address of system contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @dev TODO all the address
contract ReservedAddrConstant {
    address constant sysConfigAddr = 0xFFfffFFfFfFffFFfFFfffFffFfFFFffFFf020000;
    address constant adminAddr = 0xFFFfFFfFfFFFfFfFfFFfFFFffFFFffFFFf02000c;

    address constant permissionManagementAddr = 0xffFffFffFFffFFFFFfFfFFfFFFFfffFFff020004;
    address constant roleManagementAddr = 0xFFFFfFfFFFFFFfFfffFfffffffFffFFffF020007;
    address constant permissionCreatorAddr = 0xffFFFffFfFFffffFffffFFfFffffFfFFFF020005;
    address constant authorizationAddr = 0xFFfFffFfffFFFFFfFfFfffFFfFfFfFFfFf020006;
    address constant roleCreatorAddr = 0xffFfffffFfffFffFFFfFfffffffFfFffFF020008;
    address constant roleAuthAddr = 0xFFfFFFffffFFfffFfFFffFfFfFFfffFfFF02000d;


    address constant rootGroupAddr = 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009;
    address constant userManagementAddr = 0xFFFffFFfffffFFfffFFffffFFFffFfFffF02000a;
    address constant groupCreatorAddr = 0xfFFffFfFFFFfFFFfFfffffFFfffffffffF02000B;
    address constant allGroupsAddr = 0xfFFffFFFfffFfFFFfFfFFfffffffFfFfFf020012;

    // permission addresses
    address constant sendTxAddr = 0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000;
    address constant createContractAddr = 0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001;

    address constant approveNodeAddr = 0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020;
    address constant deleteNodeAddr = 0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021;
    address constant setStakeAddr = 0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022;
    address constant newPermissionAddr = 0xfFfFffFffffFFfffFfFfFffFFFfFFfFFFf021010;
    address constant deletePermissionAddr = 0xFFfFfffffFFffFfffFffffffFFfFfFfFfF021011;
    address constant updatePermissionNameAddr = 0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012;
    address constant addResourcesAddr = 0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012;
    address constant deleteResourcesAddr = 0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012;
    address constant setAuthorizationsAddr = 0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013;
    address constant setAuthorizationAddr = 0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013;
    address constant cancelAuthorizationsAddr = 0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014;
    address constant cancelAuthorizationAddr = 0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014;
    address constant clearAuthorizationAddr = 0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014;
    address constant newRoleAddr = 0xFFFFFfffffFFFfFfffffFfFfffffFFffFf021015;
    address constant deleteRoleAddr = 0xfFfFFFFFffFFfFFfFFfFFfFfFFfffFFffF021016;
    address constant updateRoleNameAddr = 0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017;
    address constant addPermissionsAddr = 0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017;
    address constant deletePermissionsAddr = 0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017;
    address constant setRoleAddr = 0xfFFFfFfFFFFFFffFfFFFFfffFffFfFFFFF021018;
    address constant cancelRoleAddr = 0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019;
    address constant clearRoleAddr = 0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019;
    address constant newGroupAddr = 0xFFFFffffffffFFfFffFffFFfFfFfFffFFf02101A;
    address constant deleteGroupAddr = 0xFFfFfffFffffffffFFfFfFFFFfFFfFfFFF02101B;
    address constant updateGroupNameAddr = 0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c;
    address constant addAccountsAddr = 0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c;
    address constant deleteAccountsAddr = 0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c;
    address constant setBQLAddr = 0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024;
    address constant setDefaultAQLAddr = 0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023;
    address constant setAQLAddr = 0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023;
    address constant setQuotaPriceAddr = 0xffFFffFFffFFfFFFFffffFfFFFFFFFffFf021027;
    address constant multiTxsAddr = 0xFFFffFFFfFfFFffffffFfFfFFfFfffFFFf021025;
    address constant setStateAddr = 0xfFFfffFfFfffFffFFFfFfFFfFffFFfFFFf021026;
    address constant setVersionAddr = 0xFffFffFffFfFFfFfffFffffffffFffFfFF021028;
}
