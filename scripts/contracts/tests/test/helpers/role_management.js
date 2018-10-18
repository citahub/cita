const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { roleManagement } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/RoleManagement.abi'));

const contract = genContract(abi, roleManagement);

// tmp
let param;

// newRole
const newRole = async (name, permissions, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.newRole(
    name,
    permissions,
  ).send(param);
};

// updateRoleName
const updateRoleName = async (role, name, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.updateRoleName(
    role,
    name,
  ).send(param);
};

// addPermissions
const addPermissions = async (role, permissions, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.addPermissions(
    role,
    permissions,
  ).send(param);
};

// deletePermissions
const deletePermissions = async (role, permissions, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.deletePermissions(
    role,
    permissions,
  ).send(param);
};

// setRole
const setRole = async (account, role, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.setRole(
    account,
    role,
  ).send(param);
};

// cancelRole
const cancelRole = async (account, role, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.cancelRole(
    account,
    role,
  ).send(param);
};

// clearRole
const clearRole = async (account, role, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.clearRole(account).send(param);
};

// deleteRole
const deleteRole = async (account, role, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.deleteRole(account).send(param);
};

// queryRoles
const queryRoles = account => contract.methods.queryRoles(account).call('pending');

// queryAccounts
const queryAccounts = account => contract.methods.queryAccounts(account).call('pending');

// queryPermissions
const queryPermissions = role => contract.methods.queryPermissions(role).call('pending');

module.exports = {
  newRole,
  updateRoleName,
  addPermissions,
  deletePermissions,
  setRole,
  cancelRole,
  clearRole,
  deleteRole,
  queryRoles,
  queryAccounts,
  queryPermissions,
};
