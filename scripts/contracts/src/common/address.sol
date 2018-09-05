pragma solidity ^0.4.24;


/// @title The address of system contract
/// @author ["Cryptape Technologies <contact@cryptape.com>"]
/// @dev TODO all the address
contract ReservedAddress {
    address public sysConfigAddr = 0xFFfffFFfFfFffFFfFFfffFffFfFFFffFFf020000;
    address public adminAddr = 0xFFFfFFfFfFFFfFfFfFFfFFFffFFFffFFFf02000c;

    address public permissionManagementAddr = 0xffFffFffFFffFFFFFfFfFFfFFFFfffFFff020004;
    address public roleManagementAddr = 0xFFFFfFfFFFFFFfFfffFfffffffFffFFffF020007;
    address public permissionCreatorAddr = 0xffFFFffFfFFffffFffffFFfFffffFfFFFF020005;
    address public authorizationAddr = 0xFFfFffFfffFFFFFfFfFfffFFfFfFfFFfFf020006;
    address public roleCreatorAddr = 0xffFfffffFfffFffFFFfFfffffffFfFffFF020008;
    address public roleAuthAddr = 0xFFfFFFffffFFfffFfFFffFfFfFFfffFfFF02000d;

    address public newPermissionAddr = 0xfFfFffFffffFFfffFfFfFffFFFfFFfFFFf021010;
    address public deletePermissionAddr = 0xFFfFfffffFFffFfffFffffffFFfFfFfFfF021011;
    address public updatePermissionAddr = 0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012;
    address public setAuthAddr = 0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013;
    address public cancelAuthAddr = 0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014;
    address public newRoleAddr = 0xFFFFFfffffFFFfFfffffFfFfffffFFffFf021015;
    address public deleteRoleAddr = 0xfFfFFFFFffFFfFFfFFfFFfFfFFfffFFffF021016;
    address public updateRoleAddr = 0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017;
    address public setRoleAddr = 0xfFFFfFfFFFFFFffFfFFFFfffFffFfFFFFF021018;
    address public cancelRoleAddr = 0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019;
    address public newGroupAddr = 0xFFFFffffffffFFfFffFffFFfFfFfFffFFf02101A;
    address public deleteGroupAddr = 0xFFfFfffFffffffffFFfFfFFFFfFFfFfFFF02101B;
    address public updateGroupAddr = 0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c;
    address public sendTxAddr = 0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000;
    address public createContractAddr = 0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001;

    address public rootGroupAddr = 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009;
    address public userManagementAddr = 0xFFFffFFfffffFFfffFFffffFFFffFfFffF02000a;
    address public groupCreatorAddr = 0xfFFffFfFFFFfFFFfFfffffFFfffffffffF02000B;

    address public newNodeAddr =  0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020;
    address public deleteNodeAddr =  0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021;
    address public updateNodeAddr =  0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022;

    address public accountQuotaAddr = 0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023;
    address public blockQuotaAddr = 0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024;

    address public emergencyBrakeAddr = 0xFffffFffFFFFfFFFFfFffFFFFFfFfFFfFF02000f;
    address public versionManagerAddr = 0xFffFFFfffffFFfFFFFFffffFFfFfFfffFf020011;

    address[20] builtInPermissions = [
        0xfFfFffFffffFFfffFfFfFffFFFfFFfFFFf021010,       // 0 - newPermission
        0xFFfFfffffFFffFfffFffffffFFfFfFfFfF021011,       // 1 - deletePermission
        0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012,       // 2 - addResources, deleteResources, updatePermissionName
        0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013,       // 3 - setAuthorization
        0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014,       // 4 - cancelAuthorization, clearAuthorization, cancelAuthorizations
        0xFFFFFfffffFFFfFfffffFfFfffffFFffFf021015,       // 5 - newRole
        0xfFfFFFFFffFFfFFfFFfFFfFfFFfffFFffF021016,       // 6 - deleteRole
        0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017,       // 7 - addPermissions, deletePermissions, updateRoleName
        0xfFFFfFfFFFFFFffFfFFFFfffFffFfFFFFF021018,       // 8 - setRole
        0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019,       // 9 - cancelRole, clearRole
        0xFFFFffffffffFFfFffFffFFfFfFfFffFFf02101A,       // 10 - newGroup
        0xFFfFfffFffffffffFFfFfFFFFfFFfFfFFF02101B,       // 11 - deleteGroup
        0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c,       // 12 - addAccounts, deleteAccounts, updateGroupName
        0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000,       
        0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001,
        0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020,       // 15 - approveNode
        0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021,       // 16 - deleteNOde
        0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022,       // 17 - setStake
        0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023,       // 18 - setDefaultAQL, setAQL
        0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024        // 19 - setBQL
    ];
}
