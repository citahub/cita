pub const BUILD_IN_PERMS: [&'static str; 24] = [
    "0xfFfFffFffffFFfffFfFfFffFFFfFFfFFFf021010", // 0 - newPermission
    "0xFFfFfffffFFffFfffFffffffFFfFfFfFfF021011", // 1 - deletePermission
    "0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012", // 2 - addResources, deleteResources, updatePermissionName
    "0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013", // 3 - setAuthorization
    "0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014", // 4 - cancelAuthorization, clearAuthorization, cancelAuthorizations
    "0xFFFFFfffffFFFfFfffffFfFfffffFFffFf021015", // 5 - newRole
    "0xfFfFFFFFffFFfFFfFFfFFfFfFFfffFFffF021016", // 6 - deleteRole
    "0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017", // 7 - addPermissions, deletePermissions, updateRoleName
    "0xfFFFfFfFFFFFFffFfFFFFfffFffFfFFFFF021018", // 8 - setRole
    "0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019", // 9 - cancelRole, clearRole
    "0xFFFFffffffffFFfFffFffFFfFfFfFffFFf02101A", // 10 - newGroup
    "0xFFfFfffFffffffffFFfFfFFFFfFFfFfFFF02101B", // 11 - deleteGroup
    "0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c", // 12 - addAccounts, deleteAccounts, updateGroupName
    "0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000", // 13 - sendTx
    "0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001", // 14 - createContract
    "0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020", // 15 - approveNode
    "0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021", // 16 - deleteNOde
    "0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022", // 17 - setStake
    "0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023", // 18 - setDefaultAQL, setAQL
    "0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024", // 19 - setBQL
    "0xFFFffFFFfFfFFffffffFfFfFFfFfffFFFf021025", // 20 - multiTxs
    "0xfFFfffFfFfffFffFFFfFfFFfFffFFfFFFf021026", // 21 - setState
    "0xffFFffFFffFFfFFFFffffFfFFFFFFFffFf021027", // 22 - setQuotaPrice
    "0xFffFffFffFfFFfFfffFffffffffFffFfFF021028", // 23 - setVersion
];

pub const ROOT_GROUP_ADDRESS: &'static str = "0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009";
pub const SEND_TX_ADDRESS: &'static str = "0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000";
pub const CREATE_CONTRACT_ADDRESS: &'static str = "0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001";
