/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { gmABI, gmAddr } = config.contract.group_management;

// permission management
const gManagement = web3.eth.contract(gmABI);
const gManagementContractIns = gManagement.at(gmAddr);

const quota = util.quota;
const blockLimit = util.blockLimit;

const tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: web3.eth.blockNumber + blockLimit,
    from: sender.address
};

// newPermission
const newGroup = function (parent, addrs, accounts, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.newGroup.sendTransaction(
            parent,
            addrs,
            accounts,
            tx_params
        );
};

// deleteGroup
const deleteGroup = function (origin, target, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.deleteGroup.sendTransaction(
            origin,
            target,
            tx_params
        );
};

// updateGroupName
const updateGroupName = function (origin, target, name, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.updateGroupName.sendTransaction(
            origin,
            target,
            name,
            tx_params
        );
};

// addAccounts
const addAccounts = function (origin, target, accounts, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.addAccounts.sendTransaction(
            origin,
            target,
            accounts,
            tx_params
        );
};

// deleteAccounts
const deleteAccounts = function (origin, target, accounts, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.deleteAccounts.sendTransaction(
            origin,
            target,
            accounts,
            tx_params
        );
};

// checkScope
const checkScope = function (origin, target) {
    return gManagementContractIns.checkScope.call(origin, target);
}

// checkScope
const queryGroups = function () {
    return gManagementContractIns.queryGroups.call();
}

module.exports = {
    newGroup,
    updateGroupName,
    addAccounts,
    deleteAccounts,
    deleteGroup,
    checkScope,
    queryGroups
};
