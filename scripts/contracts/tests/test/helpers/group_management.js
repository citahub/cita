const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { groupManagement } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/GroupManagement.abi'));

const contract = genContract(abi, groupManagement);

// newPermission
const newGroup = async (origin, name, accounts, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.newGroup(
    origin,
    name,
    accounts,
  ).send(param);
};

// deleteGroup
const deleteGroup = async (origin, target, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.deleteGroup(
    origin,
    target,
  ).send(param);
};

// updateGroupName
const updateGroupName = async (origin, target, name, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.updateGroupName(
    origin,
    target,
    name,
  ).send(param);
};

// addAccounts
const addAccounts = async (origin, target, accounts, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.addAccounts(
    origin,
    target,
    accounts,
  ).send(param);
};

// deleteAccounts
const deleteAccounts = async (origin, target, accounts, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.deleteAccounts(
    origin,
    target,
    accounts,
  ).send(param);
};

// checkScope
const checkScope = async (origin, target) => contract.methods.checkScope(origin, target).call('pending');

// queryGroups
const queryGroups = () => contract.methods.queryGroups().call('pending');

module.exports = {
  newGroup,
  updateGroupName,
  addAccounts,
  deleteAccounts,
  deleteGroup,
  checkScope,
  queryGroups,
};
