/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const { aABI, aAddr, superAdmin, permissions, resources } = config.contract.authorization;

// authorization
const auth = web3.eth.contract(aABI);
const aContractInstance = auth.at(aAddr);

// queryPermissions
const queryPermissions = function (account) {
    return aContractInstance.queryPermissions.call(account);
};

// queryAccounts
const queryAccounts = function (perm) {
    return aContractInstance.queryAccounts.call(perm);
};

// checkPermission
const checkPermission = function (account, addr, func) {
    return aContractInstance.checkPermission.call(
                account,
                addr,
                func
            );
};

// queryAllAccounts
const queryAllAccounts = function () {
    return aContractInstance.queryAllAccounts.call();
};

module.exports = {
    queryPermissions,
    queryAccounts,
    checkPermission,
    queryAllAccounts
};
