/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { rmABI, rmAddr, permissions } = config.contract.role_management;

const roleManagement = web3.eth.contract(rmABI);
const rmContractInstance = roleManagement.at(rmAddr);

const quota = util.quota;
const blockLimit = util.blockLimit;

let tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: web3.eth.blockNumber + blockLimit,
    from: sender.address
};

// newRole
const newRole = function (name, permissions, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.newRole.sendTransaction(
                name,
                permissions,
                tx_params
            );
};

// updateRoleName
const updateRoleName = function (role, name, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.updateRoleName.sendTransaction(
                role,
                name,
                tx_params
            );
};

// addPermissions
const addPermissions = function (role, permissions, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.addPermissions.sendTransaction(
                role,
                permissions,
                tx_params
            );
};

// deletePermissions
const deletePermissions = function (role, permissions, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.deletePermissions.sendTransaction(
                role,
                permissions,
                tx_params
            );
};

// setRole
const setRole = function (account, role, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.setRole.sendTransaction(
                account,
                role,
                tx_params
            );
};

// cancelRole
const cancelRole = function (account, role, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.cancelRole.sendTransaction(
                account,
                role,
                tx_params
            );
};

// clearRole
const clearRole = function (account, role, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.clearRole.sendTransaction(
                account,
                tx_params
            );
};

// deleteRole
const deleteRole = function (account, role, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return rmContractInstance.deleteRole.sendTransaction(
                account,
                tx_params
            );
};

// queryRoles
const queryRoles = function (account, _sender = sender) {
    return rmContractInstance.queryRoles.call(account);
};

// queryAccounts
const queryAccounts = function (account, _sender = sender) {
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
