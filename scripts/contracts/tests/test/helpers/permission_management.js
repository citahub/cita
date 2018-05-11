/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { pManagementABI, pManagementAddr } = config.contract.permission_management;

// permission management
const pManagement = web3.eth.contract(pManagementABI);
const pManagementContractIns = pManagement.at(pManagementAddr);

const genTxParams = util.genTxParams;

// newPermission
const newPermission = function (name, addrs, funcs, _sender = sender) {
    return pManagementContractIns.newPermission.sendTransaction(
                name,
                addrs,
                funcs,
                genTxParams(_sender)
            );
};

// updatePermissionName
const updatePermissionName = function (perm, name, _sender = sender) {
    return pManagementContractIns.updatePermissionName.sendTransaction(
                perm,
                name,
                genTxParams(_sender)
            );
};

// addResources
const addResources = function (perm, addrs, funcs, _sender = sender) {
    return pManagementContractIns.addResources.sendTransaction(
                perm,
                addrs,
                funcs,
                genTxParams(_sender)
            );
};

// deleteResources
const deleteResources = function (perm, addrs, funcs, _sender = sender) {
    return pManagementContractIns.deleteResources.sendTransaction(
                perm,
                addrs,
                funcs,
                genTxParams(_sender)
            );
};

// clearAuthorization
const clearAuthorization = function (account, _sender = sender) {
    return pManagementContractIns.clearAuthorization.sendTransaction(
                account,
                genTxParams(_sender)
            );
};

// setAuthorization
const setAuthorization = function (account, perm, _sender = sender) {
    return pManagementContractIns.setAuthorization.sendTransaction(
                account,
                perm,
                genTxParams(_sender)
            );
};

// cancelAuthorization
const cancelAuthorization = function (account, perm, _sender = sender) {
    return pManagementContractIns.cancelAuthorization.sendTransaction(
                account,
                perm,
                genTxParams(_sender)
            );
};

// deletePermission
const deletePermission = function (name, _sender = sender) {
    return pManagementContractIns.deletePermission.sendTransaction(
                name,
                genTxParams(_sender)
            );
};

// setAuthorizations
const setAuthorizations = function (account, perms, _sender = sender) {
    return pManagementContractIns.setAuthorizations.sendTransaction(
                account,
                perms,
                genTxParams(_sender)
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
