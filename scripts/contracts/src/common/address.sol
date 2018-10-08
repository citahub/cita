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

    address public sendTxAddr = 0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000;
    address public createContractAddr = 0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001;

    address public rootGroupAddr = 0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009;
    address public userManagementAddr = 0xFFFffFFfffffFFfffFFffffFFFffFfFffF02000a;
    address public groupCreatorAddr = 0xfFFffFfFFFFfFFFfFfffffFFfffffffffF02000B;
    address public allGroupsAddr = 0xfFFffFFFfffFfFFFfFfFFfffffffFfFfFf020012;


    address[24] public builtInPermissions = [
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
        0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000,       // 13 - sendTx
        0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001,       // 14 - createContract
        0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020,       // 15 - approveNode
        0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021,       // 16 - deleteNOde
        0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022,       // 17 - setStake
        0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023,       // 18 - setDefaultAQL, setAQL
        0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024,       // 19 - setBQL
        0xFFFffFFFfFfFFffffffFfFfFFfFfffFFFf021025,       // 20 - multiTxs
        0xfFFfffFfFfffFffFFFfFfFFfFffFFfFFFf021026,       // 21 - setState
        0xffFFffFFffFFfFFFFffffFfFFFFFFFffFf021027,       // 22 - setQuotaPrice
        0xFffFffFffFfFFfFfffFffffffffFffFfFF021028        // 23 - setVersion
    ];
}
