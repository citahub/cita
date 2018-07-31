const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const sender = config.superAdmin;
const { abi, addr } = config.contract.node_manager;

const contract = genContract(abi, addr);

// approveNode
const approveNode = async (node, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.approveNode(node).send(param);
};

// deleteNode
const deleteNode = async (node, _sender = sender) => {
  const param = await genTxParams(_sender);
  return contract.methods.deleteNode(node).send(param);
};

// listNode
const listNode = () => contract.methods.listNode().call();

// getStatus
const getStatus = node => contract.methods.getStatus(node).call();

module.exports = {
  approveNode,
  deleteNode,
  listNode,
  getStatus,
};
