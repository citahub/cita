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
const deleteGroup = function (group, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.deleteGroup.sendTransaction(
            group,
            tx_params
        );
};

// updateGroupName
const updateGroupName = function (group, name, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.updateGroupName.sendTransaction(
            group,
            name,
            tx_params
        );
};

// addAccounts
const addAccounts = function (group, accounts, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.addAccounts.sendTransaction(
            group,
            accounts,
            tx_params
        );
};

// deleteAccounts
const deleteAccounts = function (group, accounts, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.deleteAccounts.sendTransaction(
            group,
            accounts,
            tx_params
        );
};

// addChildGroup
const addChildGroup = function (group, child, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return gManagementContractIns.addChildGroup.sendTransaction(
            group,
            child,
            tx_params
        );
};

module.exports = {
    newGroup,
    updateGroupName,
    addAccounts,
    deleteAccounts,
    addChildGroup,
    deleteGroup
};
