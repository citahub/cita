/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { rmABI, rmAddr, permissions } = config.contract.role_management;

const roleManagement = web3.eth.contract(rmABI);
const rmContractInstance = roleManagement.at(rmAddr);

const genTxParams = util.genTxParams;

// newRole
const newRole = function (name, permissions, _sender = sender) {
    return rmContractInstance.newRole.sendTransaction(
                name,
                permissions,
                genTxParams(_sender)
            );
};

// updateRoleName
const updateRoleName = function (role, name, _sender = sender) {
    return rmContractInstance.updateRoleName.sendTransaction(
                role,
                name,
                genTxParams(_sender)
            );
};

// addPermissions
const addPermissions = function (role, permissions, _sender = sender) {
    return rmContractInstance.addPermissions.sendTransaction(
                role,
                permissions,
                genTxParams(_sender)
            );
};

// deletePermissions
const deletePermissions = function (role, permissions, _sender = sender) {
    return rmContractInstance.deletePermissions.sendTransaction(
                role,
                permissions,
                genTxParams(_sender)
            );
};

// setRole
const setRole = function (account, role, _sender = sender) {
    return rmContractInstance.setRole.sendTransaction(
                account,
                role,
                genTxParams(_sender)
            );
};

// cancelRole
const cancelRole = function (account, role, _sender = sender) {
    return rmContractInstance.cancelRole.sendTransaction(
                account,
                role,
                genTxParams(_sender)
            );
};

// clearRole
const clearRole = function (account, role, _sender = sender) {
    return rmContractInstance.clearRole.sendTransaction(
                account,
                genTxParams(_sender)
            );
};

// deleteRole
const deleteRole = function (account, role, _sender = sender) {
    return rmContractInstance.deleteRole.sendTransaction(
                account,
                genTxParams(_sender)
            );
};

// queryRoles
const queryRoles = function (account) {
    return rmContractInstance.queryRoles.call(account);
};

// queryAccounts
const queryAccounts = function (account) {
    return rmContractInstance.queryAccounts.call(account);
};

// queryPermissions
const queryPermissions = function (role) {
    return rmContractInstance.queryPermissions.call(role);
};

module.exports = {
    newRole,
    updateRoleName,
    addPermissions,
    deletePermissions,
    setRole,
    cancelRole,
    clearRole,
    deleteRole,
    queryRoles,
    queryAccounts,
    queryPermissions
};
