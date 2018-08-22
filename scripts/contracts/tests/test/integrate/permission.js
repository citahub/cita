const chai = require('chai');
const util = require('../helpers/util');
const permissionManagement = require('../helpers/permission_management');
const config = require('../config');

const { expect } = chai;
const {
  nervos, logger, genTxParams, genContract, getTxReceipt, web3,
} = util;


// tmp
let addr;
let hash;
let param;
let newPermissionAddr;
let contract;

// test data
const bin = '608060405234801561001057600080fd5b5060fd8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680630c55699c14604e5780632d910f2c146076575b600080fd5b348015605957600080fd5b506060608a565b6040518082815260200191505060405180910390f35b348015608157600080fd5b5060886090565b005b60005481565b600160008082825401925050819055506000547f11c1a8e7158fead62641b1e07f61c32daccb5a0432cabfe33a43e8de610042f160405160405180910390a25600a165627a7a7230582021264d3aa498b31d10a5a7086d3e3ba4fb8c23f5a30b64ef8426b19ae2de29870029';
const abi = [{
  constant: true, inputs: [], name: 'x', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  constant: false, inputs: [], name: 'AddOne', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}, {
  anonymous: false, inputs: [{ indexed: true, name: 'x', type: 'uint256' }], name: 'OneAdded', type: 'event',
}];
const setHash = '0x2d910f2c';

// permission management
const {
  setAuthorization, newPermission,
} = permissionManagement;


const name = web3.utils.utf8ToHex('testPermission');

const {
  superAdmin,
} = config;

describe('\n\nDeploy a contract\n\n', () => {
  it('should send a tx: deploy_contract', async () => {
    param = await genTxParams(superAdmin);
    const res = await nervos.appchain.deploy(
      bin,
      param,
    );
    logger.debug('\nDeploy a contract:\n', res.contractAddress);
    addr = res.contractAddress;
  });

  it('should send a tx: AddOne', async () => {
    contract = genContract(abi, addr);
    const res = await contract.methods.AddOne().send(param);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.equal('No Call contract permission.');
  });

  it('should send a tx: newPermission', async () => {
    const res = await newPermission(name, [addr], [setHash]);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
    newPermissionAddr = res.logs[0].address;
    logger.debug('\nThe new permission contract address:\n', newPermissionAddr);
  });

  it('should send a tx: setAuthorization', async () => {
    const res = await setAuthorization(superAdmin.address, newPermissionAddr);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
    setTimeout(() => {}, 10000);
  });

  it('should get receipt', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('should send a tx: AddOne', async () => {
    param = await genTxParams(superAdmin);
    const res = await contract.methods.AddOne().send(param);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });
});

