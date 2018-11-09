const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { roleAuth } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/RoleAuth.abi'));

const contract = genContract(abi, roleAuth);

// queryRoles
const queryRoles = account => contract.methods.queryRoles(account).call('pending');

// queryAccounts
const queryAccounts = account => contract.methods.queryAccounts(account).call('pending');

// queryPermissions
const queryPermissions = role => contract.methods.queryPermissions(role).call('pending');

// hasPermission
const hasPermission = (account, permission) => contract.methods.hasPermission(
  account,
  permission,
).call('pending');

module.exports = {
  queryRoles,
  queryAccounts,
  queryPermissions,
  hasPermission,
};
