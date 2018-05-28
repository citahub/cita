const util = require('./util');
const config = require('../config');

const { web3, genTxParams } = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr } = config.contract.chain_manager;

const sidechain = web3.eth.contract(abi);
const sContractInstance = sidechain.at(addr);

// newSideChain
const newSideChain = function newSideChain(address, _sender = sender) {
  return sContractInstance.newSideChain.sendTransaction(
    address,
    genTxParams(_sender),
  );
};

// enableSideChain
const enableSideChain = function enableSideChain(id, _sender = sender) {
  return sContractInstance.enableSideChain.sendTransaction(
    id,
    genTxParams(_sender),
  );
};

// disableSideChain
const disableSideChain = function disableSideChain(id, _sender = sender) {
  return sContractInstance.disableSideChain.sendTransaction(
    id,
    genTxParams(_sender),
  );
};

// getChainId
const getChainId = function getChainId() {
  return sContractInstance.getChainId.call();
};

// getParentChainId
const getParentChainId = function getParentChainId() {
  return sContractInstance.getParentChainId.call();
};

// Get the nodes of side chain
const getAuthorities = function getAuthoriti(id) {
  return sContractInstance.getAuthoritirs.call(id);
};

module.exports = {
  newSideChain,
  enableSideChain,
  disableSideChain,
  getChainId,
  getParentChainId,
  getAuthorities,
};
