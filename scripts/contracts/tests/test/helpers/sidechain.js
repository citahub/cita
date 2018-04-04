/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr } = config.contract.sidechain;

const sidechain = web3.eth.contract(abi);
const sContractInstance = sidechain.at(addr);

const quota = util.quota;
const blockLimit = util.blockLimit;

const tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: web3.eth.blockNumber + blockLimit,
    from: sender.address
};

// newChain
const newChain = function (address, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return sContractInstance.newChain.sendTransaction(
            address,
            tx_params
        );
};

// enableChain
const enableChain = function (id, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return sContractInstance.enableChain.sendTransaction(
            id,
            tx_params
        );
};

// disableChain
const disableChain = function (id, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return sContractInstance.disableChain.sendTransaction(
            id,
            tx_params
        );
};

// Get the status of side chain
const getStatus = function (id) {
    return sContractInstance.getStatus.call(id);
}

// Get the nodes of side chain
const getNodes = function (id) {
    return sContractInstance.getNodes.call(id);
}

// Get the num of side chain
const getId = function () {
    return sContractInstance.getId.call();
}

module.exports = {
    newChain,
    enableChain,
    disableChain,
    getStatus,
    getNodes,
    getId
};
