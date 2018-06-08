const util = require('./util');
const config = require('../config');

const { web3, genTxParams } = util;

const sender = config.contract.authorization.superAdmin;
const { abi, addr } = config.contract.node_manager;

const nodeAbi = web3.eth.contract(abi);
const nodeContractIns = nodeAbi.at(addr);

// addAdmin
const addAdmin = function addAdmin(account, _sender = sender) {
  return nodeContractIns.addAdmin.sendTransaction(
    account,
    genTxParams(_sender),
  );
};

// newNode
const newNode = function newNode(node, _sender = sender) {
  return nodeContractIns.newNode.sendTransaction(
    node,
    genTxParams(_sender),
  );
};

// approveNode
const approveNode = function approveNode(node, _sender = sender) {
  return nodeContractIns.approveNode.sendTransaction(
    node,
    genTxParams(_sender),
  );
};

// deleteNode
const deleteNode = function deleteNode(node, _sender = sender) {
  return nodeContractIns.deleteNode.sendTransaction(
    node,
    genTxParams(_sender),
  );
};

// listNode
const listNode = function listNode() {
  return nodeContractIns.listNode.call();
};

// getStatus
const getStatus = function getStatus(node) {
  return nodeContractIns.getStatus.call(node);
};

// isAdmin
const isAdmin = function isAdmin(account) {
  return nodeContractIns.isAdmin.call(account);
};

module.exports = {
  addAdmin,
  newNode,
  approveNode,
  deleteNode,
  listNode,
  getStatus,
  isAdmin,
};
