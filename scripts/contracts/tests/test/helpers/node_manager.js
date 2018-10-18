const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { nodeManager } = config.contract;
const abi = JSON.parse(fs.readFileSync('abi/NodeManager.abi'));

const contract = genContract(abi, nodeManager);

// approveNode
const approveNode = async (node, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.approveNode(node).send(param);
};

// deleteNode
const deleteNode = async (node, _sender = superAdmin) => {
  const param = await genTxParams(_sender);
  return contract.methods.deleteNode(node).send(param);
};

// listNode
const listNode = () => contract.methods.listNode().call('pending');

// getStatus
const getStatus = node => contract.methods.getStatus(node).call('pending');

module.exports = {
  approveNode,
  deleteNode,
  listNode,
  getStatus,
};
