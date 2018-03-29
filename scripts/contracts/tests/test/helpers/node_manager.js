/* jshint esversion: 6 */
/* jshint expr: true */

const util = require('./util');
const config = require('../config');
const web3 = util.web3;

const sender = config.contract.authorization.superAdmin;
const { abi, addr, admin } = config.contract.node_manager;

const node = web3.eth.contract(abi);
const nodeContractIns = node.at(addr);

const quota = util.quota;
const blockLimit = util.blockLimit;

// TODO refactor: Add an interface: setParams(sender)
const tx_params = {
    privkey: sender.privkey,
    nonce: util.randomInt(),
    quota,
    validUntilBlock: web3.eth.blockNumber + blockLimit,
    from: sender.address
};

// addAdmin
const addAdmin = function (account, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return nodeContractIns.addAdmin.sendTransaction(
            account,
            tx_params
        );
};

// newNode
const newNode = function (node, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return nodeContractIns.newNode.sendTransaction(
            node,
            tx_params
        );
};

// approveNode
const approveNode = function (node, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return nodeContractIns.approveNode.sendTransaction(
            node,
            tx_params
        );
};

// deleteNode
const deleteNode = function (node, _sender = sender) {
    tx_params.nonce = util.randomInt();
    tx_params.validUntilBlock = web3.eth.blockNumber + blockLimit;
    tx_params.privkey = _sender.privkey;
    tx_params.from = _sender.address;
    return nodeContractIns.deleteNode.sendTransaction(
            node,
            tx_params
        );
};

// listNode
const listNode = function () {
    return nodeContractIns.listNode.call();
};

// getStatus
const getStatus = function (node) {
    return nodeContractIns.getStatus.call(node);
};

// isAdmin
const isAdmin = function (account) {
    return nodeContractIns.isAdmin.call(account);
};

module.exports = {
    addAdmin,
    newNode,
    approveNode,
    deleteNode,
    listNode,
    getStatus,
    isAdmin
};
