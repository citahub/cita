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

const genTxParams = util.genTxParams;

// newPermission
const newGroup = function (origin, name, accounts, _sender = sender) {
    return gManagementContractIns.newGroup.sendTransaction(
                origin,
                name,
                accounts,
                genTxParams(_sender)
            );
};

// deleteGroup
const deleteGroup = function (origin, target, _sender = sender) {
    return gManagementContractIns.deleteGroup.sendTransaction(
                origin,
                target,
                genTxParams(_sender)
            );
};

// updateGroupName
const updateGroupName = function (origin, target, name, _sender = sender) {
    return gManagementContractIns.updateGroupName.sendTransaction(
                origin,
                target,
                name,
                genTxParams(_sender)
            );
};

// addAccounts
const addAccounts = function (origin, target, accounts, _sender = sender) {
    return gManagementContractIns.addAccounts.sendTransaction(
                origin,
                target,
                accounts,
                genTxParams(_sender)
            );
};

// deleteAccounts
const deleteAccounts = function (origin, target, accounts, _sender = sender) {
    return gManagementContractIns.deleteAccounts.sendTransaction(
                origin,
                target,
                accounts,
                genTxParams(_sender)
            );
};

// checkScope
const checkScope = function (origin, target) {
    return gManagementContractIns.checkScope.call(origin, target);
};

// queryGroups
const queryGroups = function () {
    return gManagementContractIns.queryGroups.call();
};

module.exports = {
    newGroup,
    updateGroupName,
    addAccounts,
    deleteAccounts,
    deleteGroup,
    checkScope,
    queryGroups
};
