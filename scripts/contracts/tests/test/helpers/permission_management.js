const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const sender = config.superAdmin;
const { abi, addr } = config.contract.permission_management;

const contract = genContract(abi, addr);

// tmp
let param;

// newPermission
const newPermission = async (name, addrs, funcs, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.newPermission(
    name,
    addrs,
    funcs,
  ).send(param);
};

// updatePermissionName
const updatePermissionName = async (perm, name, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.updatePermissionName(
    perm,
    name,
  ).send(param);
};

// addResources
const addResources = async (perm, addrs, funcs, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.addResources(
    perm,
    addrs,
    funcs,
  ).send(param);
};

// deleteResources
const deleteResources = async (perm, addrs, funcs, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.deleteResources(
    perm,
    addrs,
    funcs,
  ).send(param);
};

// clearAuthorization
const clearAuthorization = async (account, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.clearAuthorization(account).send(param);
};

// setAuthorization
const setAuthorization = async (account, perm, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.setAuthorization(account, perm).send(param);
};

// cancelAuthorization
const cancelAuthorization = async (account, perm, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.cancelAuthorization(account, perm).send(param);
};

// deletePermission
const deletePermission = async (perm, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.deletePermission(perm).send(param);
};

// setAuthorizations
const setAuthorizations = async (account, perms, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.setAuthorizations(account, perms).send(param);
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
