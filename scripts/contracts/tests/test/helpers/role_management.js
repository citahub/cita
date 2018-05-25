const util = require('./util');
const config = require('../config');

const { web3, genTxParams } = util;

const sender = config.contract.authorization.superAdmin;
const { rmABI, rmAddr } = config.contract.role_management;

const roleManagement = web3.eth.contract(rmABI);
const rmContractInstance = roleManagement.at(rmAddr);

// newRole
const newRole = function newRole(name, permissions, _sender = sender) {
  return rmContractInstance.newRole.sendTransaction(
    name,
    permissions,
    genTxParams(_sender),
  );
};

// updateRoleName
const updateRoleName = function updateRoleName(role, name, _sender = sender) {
  return rmContractInstance.updateRoleName.sendTransaction(
    role,
    name,
    genTxParams(_sender),
  );
};

// addPermissions
const addPermissions = function addPermissions(role, permissions, _sender = sender) {
  return rmContractInstance.addPermissions.sendTransaction(
    role,
    permissions,
    genTxParams(_sender),
  );
};

// deletePermissions
const deletePermissions = function deletePermissions(role, permissions, _sender = sender) {
  return rmContractInstance.deletePermissions.sendTransaction(
    role,
    permissions,
    genTxParams(_sender),
  );
};

// setRole
const setRole = function setRole(account, role, _sender = sender) {
  return rmContractInstance.setRole.sendTransaction(
    account,
    role,
    genTxParams(_sender),
  );
};

// cancelRole
const cancelRole = function cancelRole(account, role, _sender = sender) {
  return rmContractInstance.cancelRole.sendTransaction(
    account,
    role,
    genTxParams(_sender),
  );
};

// clearRole
const clearRole = function clearRole(account, role, _sender = sender) {
  return rmContractInstance.clearRole.sendTransaction(
    account,
    genTxParams(_sender),
  );
};

// deleteRole
const deleteRole = function deleteRole(account, role, _sender = sender) {
  return rmContractInstance.deleteRole.sendTransaction(
    account,
    genTxParams(_sender),
  );
};

// queryRoles
const queryRoles = function queryRoles(account) {
  return rmContractInstance.queryRoles.call(account);
};

// queryAccounts
const queryAccounts = function queryAccounts(account) {
  return rmContractInstance.queryAccounts.call(account);
};

// queryPermissions
const queryPermissions = function queryPermissions(role) {
  return rmContractInstance.queryPermissions.call(role);
};

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
