const util = require('./util');
const config = require('../config');

const { web3 } = util;

const {
  aABI, aAddr,
} = config.contract.authorization;

// authorization
const auth = web3.eth.contract(aABI);
const aContractInstance = auth.at(aAddr);

// queryPermissions
const queryPermissions = function queryPermissions(account) {
  return aContractInstance.queryPermissions.call(account);
};

// queryAccounts
const queryAccounts = function queryAccounts(perm) {
  return aContractInstance.queryAccounts.call(perm);
};

// checkResource
const checkResource = function checkResource(account, addr, func) {
  return aContractInstance.checkResource.call(
    account,
    addr,
    func,
  );
};

// queryAllAccounts
const queryAllAccounts = function queryAllAccounts() {
  return aContractInstance.queryAllAccounts.call();
};

module.exports = {
  queryPermissions,
  queryAccounts,
  checkResource,
  queryAllAccounts,
};
