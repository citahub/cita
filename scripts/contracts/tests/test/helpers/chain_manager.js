const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const sender = config.superAdmin;
const { abi, addr } = config.contract.chain_manager;

const contract = genContract(abi, addr);

// tmp
let param;

// newSideChain
const newSideChain = async (id, address, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.newSideChain(
    id,
    address,
  ).send(param);
};

// enableSideChain
const enableSideChain = async (id, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.enableSideChain(id).send(param);
};

// disableSideChain
const disableSideChain = async (id, _sender = sender) => {
  param = await genTxParams(_sender);
  return contract.methods.disableSideChain(id).send(param);
};

// getChainId
const getChainId = () => contract.methods.getChainId().call();

// getParentChainId
const getParentChainId = () => contract.methods.getParentChainId().call();

// Get the nodes of side chain
const getAuthorities = id => contract.methods.getAuthoritirs(id).call();

module.exports = {
  newSideChain,
  enableSideChain,
  disableSideChain,
  getChainId,
  getParentChainId,
  getAuthorities,
};
