/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.permission_manager.sender;
const { pManagementABI, pManagementAddr } = config.contract.permission_management;

// permission management
const pManagement = web3.eth.contract(pManagementABI);
const pManagementContractIns = pManagement.at(pManagementAddr);

const quota = util.quota;
const blockLimit = util.blockLimit;

const tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: util.blockNumber + blockLimit,
    from: sender.address
};

// newPermission
const newPermission = function (name, addrs, funcs) {
    return pManagementContractIns.newPermission.sendTransaction(
            name,
            addrs,
            funcs,
            tx_params
        );
};

// updatePermissionName
const updatePermissionName = function (perm, name) {
    return pManagementContractIns.updatePermissionName.sendTransaction(
            perm,
            name,
            tx_params
        );
};

// addResources
const addResources = function (perm, addrs, funcs) {
    return pManagementContractIns.addResources.sendTransaction(
            perm,
            addrs,
            funcs,
            tx_params
        );
};

// deleteResources
const deleteResources = function (perm, addrs, funcs) {
    return pManagementContractIns.deleteResources.sendTransaction(
            perm,
            addrs,
            funcs,
            tx_params
        );
};

// clearAuthorization
const clearAuthorization = function (account) {
    return pManagementContractIns.clearAuthorization.sendTransaction(
            account,
            tx_params
        );
};

// setAuthorization
const setAuthorization = function (account, perm) {
    return pManagementContractIns.setAuthorization.sendTransaction(
            account,
            perm,
            tx_params
        );
};

// cancelAuthorization
const cancelAuthorization = function (account, perm) {
    return pManagementContractIns.cancelAuthorization.sendTransaction(
            account,
            perm,
            tx_params
        );
};

// deletePermission
const deletePermission = function (name) {
    return pManagementContractIns.deletePermission.sendTransaction(
            name,
            tx_params
        );
};

// setAuthorizations
const setAuthorizations = function (account, perms) {
    return pManagementContractIns.setAuthorizations.sendTransaction(
            account,
            perms,
            tx_params
        );
};

module.exports = {
    newPermission,
    updatePermissionName,
    addResources,
    deleteResources,
    clearAuthorization,
    setAuthorization,
    cancelAuthorization,
    deletePermission,
    setAuthorizations
};
