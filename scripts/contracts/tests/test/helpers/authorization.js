const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { authorization } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/Authorization.abi'));
const contract = genContract(abi, authorization);

// queryPermissions
const queryPermissions = account => contract.methods.queryPermissions(account).call('pending');

// queryAccounts
const queryAccounts = perm => contract.methods.queryAccounts(perm).call('pending');

// checkPermission
const checkPermission = (account, permission) => contract.methods.checkPermission(
  account,
  permission,
).call('pending');

// queryAllAccounts
const queryAllAccounts = () => contract.methods.queryAllAccounts().call('pending');

module.exports = {
  queryPermissions,
  queryAccounts,
  checkPermission,
  queryAllAccounts,
};
