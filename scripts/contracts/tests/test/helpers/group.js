/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const { gABI, gAddr} = config.contract.group;

const group = web3.eth.contract(gABI);
const gContractInstance = group.at(gAddr);

// queryInfo
const queryInfo = function () {
    return gContractInstance.queryInfo.call();
};

// queryAccounts
const queryAccounts = function () {
    return gContractInstance.queryAccounts.call();
};

// queryParent
const queryParent = function () {
    return gContractInstance.queryParent.call();
};

module.exports = {
    group,
    queryInfo,
    queryAccounts,
    queryParent
};
