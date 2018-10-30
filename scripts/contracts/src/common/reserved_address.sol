pragma solidity ^0.4.24;

/// @title The address of system contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @dev TODO all the address
contract ReservedAddress {
    address constant sysConfigAddr = 0xFFfffFFfFfFffFFfFFfffFffFfFFFffFFf020000;
    address constant adminAddr = 0xFFFfFFfFfFFFfFfFfFFfFFFffFFFffFFFf02000c;

    address constant permissionManagementAddr = 0xffFffFffFFffFFFFFfFfFFfFFFFfffFFff020004;
    address constant roleManagementAddr = 0xFFFFfFfFFFFFFfFfffFfffffffFffFFffF020007;
    address constant permissionCreatorAddr = 0xffFFFffFfFFffffFffffFFfFffffFfFFFF020005;
    address constant authorizationAddr = 0xFFfFffFfffFFFFFfFfFfffFFfFfFfFFfFf020006;
    address constant roleCreatorAddr = 0xffFfffffFfffFffFFFfFfffffffFfFffFF020008;
    address constant roleAuthAddr = 0xFFfFFFffffFFfffFfFFffFfFfFFfffFfFF02000d;

    address constant sendTxAddr = 0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000;
    address constant createContractAddr = 0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001;

    address constant rootGroupAddr = 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009;
    address constant userManagementAddr = 0xFFFffFFfffffFFfffFFffffFFFffFfFffF02000a;
    address constant groupCreatorAddr = 0xfFFffFfFFFFfFFFfFfffffFFfffffffffF02000B;
    address constant allGroupsAddr = 0xfFFffFFFfffFfFFFfFfFFfffffffFfFfFf020012;
}
