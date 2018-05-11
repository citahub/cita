/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const { pABI, pAddr} = config.contract.permission;

const perm = web3.eth.contract(pABI);
const pContractInstance = perm.at(pAddr);

// queryInfo
const queryInfo = function () {
    return pContractInstance.queryInfo.call();
};

// inPermission
const inPermission = function (addr, func) {
    return pContractInstance.inPermission.call(
                addr,
                func 
            );
};

module.exports = {
    perm,
    queryInfo,
    inPermission
};
