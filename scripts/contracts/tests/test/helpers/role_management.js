/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.permission_manager.sender;
const { rmABI, rmAddr, permissions } = config.contract.role_management;

const roleManagement = web3.eth.contract(rmABI);
const rmContractInstance = roleManagement.at(rmAddr);

const quota = util.quota;
const blockLimit = util.blockLimit;

let tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: util.blockNumber + blockLimit,
    from: sender.address
};

// newRole
const newRole = function (name, permissions) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.newRole.sendTransaction(
                name,
                permissions,
                tx_params
            );
};

// updateRoleName
const updateRoleName = function (role, name) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.updateRoleName.sendTransaction(
                role,
                name,
                tx_params
            );
};

// addPermissions
const addPermissions = function (role, permissions) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.addPermissions.sendTransaction(
                role,
                permissions,
                tx_params
            );
};

// deletePermissions
const deletePermissions = function (role, permissions) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.deletePermissions.sendTransaction(
                role,
                permissions,
                tx_params
            );
};

// setRole
const setRole = function (account, role) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.setRole.sendTransaction(
                account,
                role,
                tx_params
            );
};

// cancelRole
const cancelRole = function (account, role) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.cancelRole.sendTransaction(
                account,
                role,
                tx_params
            );
};

// clearRole
const clearRole = function (account, role) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.clearRole.sendTransaction(
                account,
                tx_params
            );
};

// deleteRole
const deleteRole = function (account, role) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = util.blockNumber + blockLimit;
    return rmContractInstance.deleteRole.sendTransaction(
                account,
                tx_params
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
    queryAccounts
};
