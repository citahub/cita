module.exports = {
  contract: {
    permission: {
      abi:
        [{
          constant: true, inputs: [{ name: 'cont', type: 'address' }, { name: 'func', type: 'bytes4' }], name: 'inPermission', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_name', type: 'bytes32' }], name: 'updateName', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryInfo', outputs: [{ name: '', type: 'bytes32' }, { name: '', type: 'address[]' }, { name: '', type: 'bytes4[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_conts', type: 'address[]' }, { name: '_funcs', type: 'bytes4[]' }], name: 'deleteResources', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryName', outputs: [{ name: '', type: 'bytes32' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [], name: 'close', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryResource', outputs: [{ name: '', type: 'address[]' }, { name: '', type: 'bytes4[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_conts', type: 'address[]' }, { name: '_funcs', type: 'bytes4[]' }], name: 'addResources', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          inputs: [{ name: '_name', type: 'bytes32' }, { name: '_conts', type: 'address[]' }, { name: '_funcs', type: 'bytes4[]' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
        }, {
          anonymous: false, inputs: [{ indexed: false, name: '_conts', type: 'address[]' }, { indexed: false, name: '_funcs', type: 'bytes4[]' }], name: 'ResourcesAdded', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: false, name: '_conts', type: 'address[]' }, { indexed: false, name: '_funcs', type: 'bytes4[]' }], name: 'ResourcesDeleted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_oldName', type: 'bytes32' }, { indexed: true, name: '_name', type: 'bytes32' }], name: 'NameUpdated', type: 'event',
        }],
      addr: '0xffffffffffffffffffffffffffffffffff021010',
    },
    authorization: {
      abi:
        [{
          constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_permission', type: 'address' }], name: 'cancelAuth', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }, { name: '_permission', type: 'address' }], name: 'checkPermission', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_permission', type: 'address' }], name: 'queryAccounts', outputs: [{ name: '_accounts', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_permission', type: 'address' }], name: 'clearAuthOfPermission', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }], name: 'queryPermissions', outputs: [{ name: '_permissions', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'clearAuth', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryAllAccounts', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }, { name: '_cont', type: 'address' }, { name: '_func', type: 'bytes4' }], name: 'checkResource', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_permission', type: 'address' }], name: 'setAuth', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          inputs: [{ name: '_superAdmin', type: 'address' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_permission', type: 'address' }], name: 'AuthSetted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_permission', type: 'address' }], name: 'AuthCanceled', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }], name: 'AuthCleared', type: 'event',
        }],
      addr: '0xffffffffffffffffffffffffffffffffff020006',
    },
    permission_management: {
      abi: [{
        constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_permission', type: 'address' }], name: 'setAuthorization', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_permission', type: 'address' }], name: 'cancelAuthorization', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_permissions', type: 'address[]' }], name: 'setAuthorizations', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_permission', type: 'address' }, { name: '_name', type: 'bytes32' }], name: 'updatePermissionName', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_permission', type: 'address' }, { name: '_conts', type: 'address[]' }, { name: '_funcs', type: 'bytes4[]' }], name: 'deleteResources', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_permission', type: 'address' }], name: 'deletePermission', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'clearAuthorization', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_permissions', type: 'address[]' }], name: 'cancelAuthorizations', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_permission', type: 'address' }, { name: '_conts', type: 'address[]' }, { name: '_funcs', type: 'bytes4[]' }], name: 'addResources', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_name', type: 'bytes32' }, { name: '_conts', type: 'address[]' }, { name: '_funcs', type: 'bytes4[]' }], name: 'newPermission', outputs: [{ name: 'id', type: 'address' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        anonymous: false, inputs: [{ indexed: false, name: '_permission', type: 'address' }], name: 'PermissionDeleted', type: 'event',
      }],
      addr: '0xffffffffffffffffffffffffffffffffff020004',
    },
    role_management: {
      abi: [{
        constant: true, inputs: [], name: 'deleteRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_role', type: 'address' }, { name: '_permissions', type: 'address[]' }], name: 'addPermissions', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'createContractAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_role', type: 'address' }, { name: '_permissions', type: 'address[]' }], name: 'deletePermissions', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'rootGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'groupCreatorAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'newPermissionAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'permissionCreatorAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'deleteGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_role', type: 'address' }], name: 'deleteRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_name', type: 'bytes32' }, { name: '_permissions', type: 'address[]' }], name: 'newRole', outputs: [{ name: 'roleid', type: 'address' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'newRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'cancelAuthAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'cancelRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'newGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'roleManagementAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'deletePermissionAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'setAuthAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'adminAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'updateRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'userManagementAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_role', type: 'address' }], name: 'setRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_role', type: 'address' }], name: 'cancelRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'updateGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'permissionManagementAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'authorizationAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'setRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'clearRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'sendTxAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_role', type: 'address' }, { name: '_name', type: 'bytes32' }], name: 'updateRoleName', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'roleAuthAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'updatePermissionAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'roleCreatorAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
      }],
      addr: '0xffffffffffffffffffffffffffffffffff020007',
    },
    role: {
      abi: [{
        constant: false, inputs: [{ name: '_permissions', type: 'address[]' }], name: 'addPermissions', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [], name: 'deleteRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [{ name: '_permission', type: 'address' }], name: 'inPermissions', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_name', type: 'bytes32' }], name: 'updateName', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'queryName', outputs: [{ name: '', type: 'bytes32' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'queryPermissions', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'queryRole', outputs: [{ name: '', type: 'bytes32' }, { name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [], name: 'lengthOfPermissions', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_permissions', type: 'address[]' }], name: 'deletePermissions', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        inputs: [{ name: '_name', type: 'bytes32' }, { name: '_permissions', type: 'address[]' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
      }, {
        anonymous: false, inputs: [{ indexed: true, name: '_oldName', type: 'bytes32' }, { indexed: true, name: '_newName', type: 'bytes32' }], name: 'NameUpdated', type: 'event',
      }, {
        anonymous: false, inputs: [{ indexed: false, name: '_permissions', type: 'address[]' }], name: 'PermissionsAdded', type: 'event',
      }, {
        anonymous: false, inputs: [{ indexed: false, name: '_permissions', type: 'address[]' }], name: 'PermissionsDeleted', type: 'event',
      }, {
        anonymous: false, inputs: [{ indexed: true, name: '_name', type: 'bytes32' }, { indexed: false, name: '_permissions', type: 'address[]' }], name: 'RoleCreated', type: 'event',
      }],
    },
    group_management: {
      abi: [{
        constant: false, inputs: [{ name: '_origin', type: 'address' }, { name: '_target', type: 'address' }, { name: '_accounts', type: 'address[]' }], name: 'addAccounts', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [], name: 'queryGroups', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_origin', type: 'address' }, { name: '_target', type: 'address' }, { name: '_name', type: 'bytes32' }], name: 'updateGroupName', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_origin', type: 'address' }, { name: '_target', type: 'address' }], name: 'deleteGroup', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_parent', type: 'address' }, { name: '_name', type: 'bytes32' }, { name: '_accounts', type: 'address[]' }], name: 'newGroup', outputs: [{ name: 'new_group', type: 'address' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: '_origin', type: 'address' }, { name: '_target', type: 'address' }, { name: '_accounts', type: 'address[]' }], name: 'deleteAccounts', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [{ name: '_origin', type: 'address' }, { name: '_target', type: 'address' }], name: 'checkScope', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        inputs: [], payable: false, stateMutability: 'nonpayable', type: 'constructor',
      }, {
        anonymous: false, inputs: [{ indexed: false, name: '_group', type: 'address' }], name: 'GroupDeleted', type: 'event',
      }],
      addr: '0xffffffffffffffffffffffffffffffffff02000a',
    },
    group: {
      abi:
        [{
          constant: false, inputs: [{ name: '_accounts', type: 'address[]' }], name: 'deleteAccounts', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_name', type: 'bytes32' }], name: 'updateName', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_child', type: 'address' }], name: 'addChild', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }], name: 'inGroup', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryInfo', outputs: [{ name: '', type: 'bytes32' }, { name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryName', outputs: [{ name: '', type: 'bytes32' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [], name: 'close', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryParent', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_child', type: 'address' }], name: 'deleteChild', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_accounts', type: 'address[]' }], name: 'addAccounts', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryChildLength', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryAccounts', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'queryChild', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          inputs: [{ name: '_parent', type: 'address' }, { name: '_name', type: 'bytes32' }, { name: '_accounts', type: 'address[]' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_parent', type: 'address' }, { indexed: true, name: '_name', type: 'bytes32' }, { indexed: false, name: '_accounts', type: 'address[]' }], name: 'GroupNewed', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: false, name: '_accounts', type: 'address[]' }], name: 'AccountsAdded', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: false, name: '_accounts', type: 'address[]' }], name: 'AccountsDeleted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_oldName', type: 'bytes32' }, { indexed: true, name: '_newName', type: 'bytes32' }], name: 'NameUpdated', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_child', type: 'address' }], name: 'ChildDeleted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_child', type: 'address' }], name: 'ChildAdded', type: 'event',
        }],
      addr: '0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009',
    },
    quota: {
      abi:
        [{
          constant: true, inputs: [], name: 'getBQL', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }], name: 'isAdmin', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_value', type: 'uint256' }], name: 'setAQL', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'addAdmin', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'getAccounts', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_value', type: 'uint256' }], name: 'setBQL', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }], name: 'getAQL', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_value', type: 'uint256' }], name: 'setDefaultAQL', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'getDefaultAQL', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'getQuotas', outputs: [{ name: '', type: 'uint256[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          inputs: [{ name: '_account', type: 'address' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: 'value', type: 'uint256' }, { indexed: true, name: '_sender', type: 'address' }], name: 'DefaultAqlSetted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: 'value', type: 'uint256' }, { indexed: true, name: '_sender', type: 'address' }], name: 'BqlSetted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_sender', type: 'address' }], name: 'AdminAdded', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: false, name: 'value', type: 'uint256' }, { indexed: true, name: '_sender', type: 'address' }], name: 'AqlSetted', type: 'event',
        }],
      addr: '0xffffffffffffffffffffffffffffffffff020003',
    },
    node_manager: {
      abi:
        [{
          constant: true, inputs: [{ name: '_node', type: 'address' }], name: 'stakePermillage', outputs: [{ name: '', type: 'uint64' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }], name: 'isAdmin', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_node', type: 'address' }], name: 'deleteNode', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_node', type: 'address' }], name: 'getStatus', outputs: [{ name: '', type: 'uint8' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_node', type: 'address' }, { name: 'stake', type: 'uint64' }], name: 'setStake', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'listNode', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '', type: 'address' }], name: 'status', outputs: [{ name: '', type: 'uint8' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'listStake', outputs: [{ name: '_stakes', type: 'uint64[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'addAdmin', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_node', type: 'address' }], name: 'approveNode', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          inputs: [{ name: '_nodes', type: 'address[]' }, { name: '_admins', type: 'address[]' }, { name: '_stakes', type: 'uint64[]' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: 'errorType', type: 'uint8' }, { indexed: false, name: 'msg', type: 'string' }], name: 'ErrorLog', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_node', type: 'address' }], name: 'ApproveNode', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_node', type: 'address' }], name: 'DeleteNode', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_sender', type: 'address' }], name: 'AddAdmin', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_node', type: 'address' }, { indexed: false, name: 'stake', type: 'uint256' }], name: 'SetStake', type: 'event',
        }],
      addr: '0xffffffffffffffffffffffffffffffffff020001',
    },
    chain_manager: {
      abi:
      [{
        constant: true, inputs: [], name: 'getChainId', outputs: [{ name: '', type: 'uint32' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: true, inputs: [{ name: 'id', type: 'uint32' }], name: 'getAuthorities', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        constant: false, inputs: [], name: 'getParentChainId', outputs: [{ name: '', type: 'uint32' }], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: 'id', type: 'uint32' }], name: 'enableSideChain', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: 'sideChainId', type: 'uint32' }, { name: 'addrs', type: 'address[]' }], name: 'newSideChain', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: false, inputs: [{ name: 'id', type: 'uint32' }], name: 'disableSideChain', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
      }, {
        constant: true, inputs: [{ name: '', type: 'uint32' }], name: 'sideChains', outputs: [{ name: 'status', type: 'uint8' }], payable: false, stateMutability: 'view', type: 'function',
      }, {
        inputs: [{ name: '_pid', type: 'uint32' }, { name: '_addrs', type: 'address[]' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
      }, {
        anonymous: false, inputs: [{ indexed: true, name: 'errorType', type: 'uint8' }, { indexed: false, name: 'msg', type: 'string' }], name: 'ErrorLog', type: 'event',
      }],
      addr: '0xffffffffffffffffffffffffffffffffff020002',
    },
    admin: {
      abi:
        [{
          constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'update', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'isAdmin', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'admin', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          inputs: [{ name: '_account', type: 'address' }], payable: false, stateMutability: 'nonpayable', type: 'constructor',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_old', type: 'address' }, { indexed: true, name: '_sender', type: 'address' }], name: 'AdminUpdated', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: 'errorType', type: 'uint8' }, { indexed: false, name: 'msg', type: 'string' }], name: 'ErrorLog', type: 'event',
        }],
      addr: '0xffffffffffffffffffffffffffffffffff02000c',
    },
    role_auth: {
      abi:
        [{
          constant: true, inputs: [], name: 'deleteRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'createContractAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_role', type: 'address' }], name: 'queryAccounts', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'rootGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'groupCreatorAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'newPermissionAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'permissionCreatorAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'deleteGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'newRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'cancelAuthAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'cancelRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'newGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'roleManagementAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'deletePermissionAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'setAuthAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_role', type: 'address' }], name: 'clearAuthOfRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'adminAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'updateRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'userManagementAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_role', type: 'address' }, { name: '_permissions', type: 'address[]' }], name: 'setPermissionsOfRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_role', type: 'address' }], name: 'setRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }, { name: '_role', type: 'address' }], name: 'cancelRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'updateGroupAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'permissionManagementAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'authorizationAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'setRoleAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_account', type: 'address' }], name: 'clearRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'sendTxAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }, { name: '_permission', type: 'address' }], name: 'hasPermission', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: false, inputs: [{ name: '_role', type: 'address' }, { name: '_permissions', type: 'address[]' }], name: 'cancelPermissionsOfRole', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'nonpayable', type: 'function',
        }, {
          constant: true, inputs: [], name: 'roleAuthAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'updatePermissionAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [], name: 'roleCreatorAddr', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          constant: true, inputs: [{ name: '_account', type: 'address' }], name: 'queryRoles', outputs: [{ name: '', type: 'address[]' }], payable: false, stateMutability: 'view', type: 'function',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_role', type: 'address' }], name: 'RoleSetted', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }, { indexed: true, name: '_role', type: 'address' }], name: 'RoleCanceled', type: 'event',
        }, {
          anonymous: false, inputs: [{ indexed: true, name: '_account', type: 'address' }], name: 'RoleCleared', type: 'event',
        }],
      addr: '0xffffffffffffffffffffffffffffffffff02000d',
    },
  },
  localServer: 'http://127.0.0.1:1337',
  remoteServer: 'http://xx.xx.xx.xx:1337',
  // TODO delete. use the real exist contract
  testAddr: ['0x1a702A25c6bcA72B67987968f0BFb3A3213c5600', '0x1a702A25c6BCA72b67987968f0BFb3a3213c5601', '0x1A702a25C6bCa72b67987968f0bfb3A3213C5602'],
  testFunc: ['0xf036ed56', '0x3482e0c9', '0xf036ed56'],
  testBin: '6060604052341561000f57600080fd5b60d38061001d6000396000f3006060604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff16806360fe47b114604e5780636d4ce63c14606e575b600080fd5b3415605857600080fd5b606c60048080359060200190919050506094565b005b3415607857600080fd5b607e609e565b6040518082815260200191505060405180910390f35b8060008190555050565b600080549050905600a165627a7a7230582020642d4bfc8bb29cfd3390d1aafea86ec7219a70889a640325d7fabb0b0534960029',
  testSender: {
    address: '0x9dcd6B234E2772C5451Fd4ccf7582f4283140697',
    privkey: '993ef0853d7bf1f4c2977457b50ea6b5f8bc2fd829e3ca3e19f6081ddabb07e9',
  },
  permissions:
    ['0xFFffFFFFfFFFFFFfffFfFFffFfFFFFfFFf021000',
      '0xffFFffffFfffFFFfffffFFfFFffFFfFFFf021001',
      '0xfFfFffFffffFFfffFfFfFffFFFfFFfFFFf021010',
      '0xFFfFfffffFFffFfffFffffffFFfFfFfFfF021011',
      '0xfFFfFFfFFFFffffFFFFFfffffFFFFFFFFf021012',
      '0xfFFFffFffFfffFffFfffFfFFfFFFfFffFf021013',
      '0xfFFFffFfffFFFFffFfFffffFfFFFfffFfF021014',
      '0xFFFFFfffffFFFfFfffffFfFfffffFFffFf021015',
      '0xfFfFFFFFffFFfFFfFFfFFfFfFFfffFFffF021016',
      '0xFFFFffFFFFfFFFFFFfFFffffFFFFFFFFff021017',
      '0xfFFFfFfFFFFFFffFfFFFFfffFffFfFFFFF021018',
      '0xfFFffffffFffFffFFFFFFFFFffFfffFFfF021019',
      '0xFFFFffffffffFFfFffFffFFfFfFfFffFFf02101A',
      '0xFFfFfffFffffffffFFfFfFFFFfFFfFfFFF02101B',
      '0xFFFfFFfffFFffFffffffFFFFFFfFFffffF02101c',
      '0xFfFfFFffFffffFffffffffFFFFffFfFFFF021020',
      '0xFffFFFfFfFFFfFfFfFfFFfffFFFFFffFFF021021',
      '0xFffFfffFFffFFFFfFFFFFfFfFFFfFFFfFF021022',
      '0xffFfffFfffFfFFFFFfFfFffFFfFfffFffF021023',
      '0xfffffFfFfFFfFFffFfFffFFFFfFFFfFffF021024',
    ],
  resources: [
    ['0xffffffffffffffffffffffffffffffffff021000', '0x00000000'],
    ['0xffffffffffffffffffffffffffffffffff021001', '0x00000000'],
    ['0xffffffffffffffffffffffffffffffffff020004', '0xfc4a089c', '0x98a05bb1', '0xf036ed56', '0x6446ebd8', '0x537bf9a3', '0x0f5aa9f3', '0x52c5b4cc', '0x3482e0c9', '0xa5925b5b', '0xba00ab60'],
    ['0xffffffffffffffffffffffffffffffffff020007', '0x551ef860', '0x54b025c5', '0x0773e6ba', '0x17b2e350', '0xd9c090a0', '0xa32710eb', '0xa8319481', '0xc631e758'],
    ['0xffffffffffffffffffffffffffffffffff02000a', '0xd7cd7209', '0xbaeb8cad', '0x2c84e31f', '0xd86df333', '0x7eafcdb1'],
  ],
  superAdmin: {
    address: '0x4b5Ae4567aD5D9FB92Bc9aFd6A657e6fA13a2523',
    privkey: '5f0258a4778057a8a7d97809bd209055b2fbafa654ce7d31ec7191066b9225e6',
  },
  rootGroup: '0xfFFfFFFFFffFFfffFFFFfffffFffffFFfF020009',
};
