const util = require('./util');
const config = require('../config');

const { web3, genTxParams } = util;

const sender = config.contract.authorization.superAdmin;
const { pManagementABI, pManagementAddr } = config.contract.permission_management;

// permission management
const pManagement = web3.eth.contract(pManagementABI);
const pManagementContractIns = pManagement.at(pManagementAddr);

// newPermission
const newPermission = function newPermission(name, addrs, funcs, _sender = sender) {
  return pManagementContractIns.newPermission.sendTransaction(
    name,
    addrs,
    funcs,
    genTxParams(_sender),
  );
};

// updatePermissionName
const updatePermissionName = function updatePermissionName(perm, name, _sender = sender) {
  return pManagementContractIns.updatePermissionName.sendTransaction(
    perm,
    name,
    genTxParams(_sender),
  );
};

// addResources
const addResources = function addResources(perm, addrs, funcs, _sender = sender) {
  return pManagementContractIns.addResources.sendTransaction(
    perm,
    addrs,
    funcs,
    genTxParams(_sender),
  );
};

// deleteResources
const deleteResources = function deleteResources(perm, addrs, funcs, _sender = sender) {
  return pManagementContractIns.deleteResources.sendTransaction(
    perm,
    addrs,
    funcs,
    genTxParams(_sender),
  );
};

// clearAuthorization
const clearAuthorization = function clearAuthorization(account, _sender = sender) {
  return pManagementContractIns.clearAuthorization.sendTransaction(
    account,
    genTxParams(_sender),
  );
};

// setAuthorization
const setAuthorization = function setAuthorization(account, perm, _sender = sender) {
  return pManagementContractIns.setAuthorization.sendTransaction(
    account,
    perm,
    genTxParams(_sender),
  );
};

// cancelAuthorization
const cancelAuthorization = function cancelAuthorization(account, perm, _sender = sender) {
  return pManagementContractIns.cancelAuthorization.sendTransaction(
    account,
    perm,
    genTxParams(_sender),
  );
};

// deletePermission
const deletePermission = function deletePermission(name, _sender = sender) {
  return pManagementContractIns.deletePermission.sendTransaction(
    name,
    genTxParams(_sender),
  );
};

// setAuthorizations
const setAuthorizations = function setAuthorizations(account, perms, _sender = sender) {
  return pManagementContractIns.setAuthorizations.sendTransaction(
    account,
    perms,
    genTxParams(_sender),
  );
};

module.exports = {
  newPermission,
  updatePermissionName,
  addResources,
  deleteResources,
  clearAuthorization,
  setAuthorization,
  cancelAuthorization,
  deletePermission,
  setAuthorizations,
};
