const util = require('./util');
const config = require('../config');

const { genContract } = util;

const { abi, addr } = config.contract.role_auth;

const contract = genContract(abi, addr);

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
