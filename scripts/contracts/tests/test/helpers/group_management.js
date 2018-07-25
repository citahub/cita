const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const sender = config.superAdmin;
const { abi, addr } = config.contract.group_management;

const contract = genContract(abi, addr);

// newPermission
const newGroup = async (origin, name, accounts, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.newGroup(
    origin,
    name,
    accounts,
  ).send(param);
};

// deleteGroup
const deleteGroup = async (origin, target, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.deleteGroup(
    origin,
    target,
  ).send(param);
};

// updateGroupName
const updateGroupName = async (origin, target, name, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.updateGroupName(
    origin,
    target,
    name,
  ).send(param);
};

// addAccounts
const addAccounts = async (origin, target, accounts, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.addAccounts(
    origin,
    target,
    accounts,
  ).send(param);
};

// deleteAccounts
const deleteAccounts = async (origin, target, accounts, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.deleteAccounts(
    origin,
    target,
    accounts,
  ).send(param);
};

// checkScope
const checkScope = async (origin, target) => contract.methods.checkScope(origin, target).call();

// queryGroups
const queryGroups = () => contract.methods.queryGroups().call();

module.exports = {
  newGroup,
  updateGroupName,
  addAccounts,
  deleteAccounts,
  deleteGroup,
  checkScope,
  queryGroups,
};
