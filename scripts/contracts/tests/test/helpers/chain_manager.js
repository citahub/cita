const fs = require('fs');
const util = require('./util');
const config = require('../config');

const { genContract, genTxParams } = util;

const { superAdmin } = config;
const { chainManager } = config.contract;

const abi = JSON.parse(fs.readFileSync('abi/ChainManager.abi'));
const contract = genContract(abi, chainManager);

// tmp
let param;

// newSideChain
const newSideChain = async (id, address, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.newSideChain(
    id,
    address,
  ).send(param);
};

// enableSideChain
const enableSideChain = async (id, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.enableSideChain(id).send(param);
};

// disableSideChain
const disableSideChain = async (id, _sender = superAdmin) => {
  param = await genTxParams(_sender);
  return contract.methods.disableSideChain(id).send(param);
};

// getChainId
const getChainId = () => contract.methods.getChainId().call('pending');

// getParentChainId
const getParentChainId = () => contract.methods.getParentChainId().call('pending');

// Get the nodes of side chain
const getAuthorities = id => contract.methods.getAuthoritirs(id).call('pending');

module.exports = {
  newSideChain,
  enableSideChain,
  disableSideChain,
  getChainId,
  getParentChainId,
  getAuthorities,
};
