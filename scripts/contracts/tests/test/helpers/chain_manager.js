/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr } = config.contract.chain_manager;

const sidechain = web3.eth.contract(abi);
const sContractInstance = sidechain.at(addr);

const genTxParams = util.genTxParams;

// newSideChain
const newSideChain = function (address, _sender = sender) {
    return sContractInstance.newSideChain.sendTransaction(
                address,
                genTxParams(_sender)
            );
};

// enableSideChain
const enableSideChain = function (id, _sender = sender) {
    return sContractInstance.enableSideChain.sendTransaction(
                id,
                genTxParams(_sender)
            );
};

// disableSideChain
const disableSideChain = function (id, _sender = sender) {
    return sContractInstance.disableSideChain.sendTransaction(
                id,
                genTxParams(_sender)
            );
};

// getChainId
const getChainId = function () {
    return sContractInstance.getChainId.call();
};

// getParentChainId
const getParentChainId = function () {
    return sContractInstance.getParentChainId.call();
};

// Get the nodes of side chain
const getAuthorities = function (id) {
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
