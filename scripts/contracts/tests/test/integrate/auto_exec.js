const chai = require('chai');
const auto = require('../helpers/auto_exec');
const util = require('../helpers/util');
const config = require('../config');

const { expect } = chai;
const {
  citaSDK, logger, genTxParams, genContract, getTxReceipt,
} = util;

const { superAdmin } = config;

const {
  register, autoExec,
} = auto;

// tmp
let addr;
let param;
let hash;

// test data
const bin = '608060405234801561001057600080fd5b506101a1806100206000396000f300608060405260043610610057576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680630c55699c1461005c5780634826c2be14610087578063844cbc43146100de575b600080fd5b34801561006857600080fd5b506100716100f5565b6040518082815260200191505060405180910390f35b34801561009357600080fd5b5061009c6100fb565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b3480156100ea57600080fd5b506100f3610121565b005b60005481565b600160009054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b600080815480929190600101919050555041600160006101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505600a165627a7a723058203e04df92be8ab872ae22ab17a37324afaea0956de6408cb798dcb2394353f1e70029';
const abi = [{
  constant: true, inputs: [], name: 'x', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  constant: true, inputs: [], name: 'coinBase', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  constant: false, inputs: [], name: 'autoExec', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}];

describe('\n\nDeploy a contract\n\n', () => {
  it('should send a tx: deploy_contract', async () => {
    param = await genTxParams(superAdmin);
    const res = await citaSDK.base.deploy(
      bin,
      param,
    );
    logger.debug('\nDeploy a contract:\n', res.contractAddress);
    addr = res.contractAddress;
  });
});

describe('\n\ntest auto exec\n\n', () => {
  before('get the x before auto exec', async () => {
    const contract = genContract(abi, addr);
    const ret = await contract.methods.x().call('pending');
    logger.debug('\nThe x:\n', ret);
    expect(ret).to.equal('0');
  });

  it('should register a auto exec tx', async () => {
    const res = await register(addr);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt:', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('should wait a block', done => setTimeout(done, 10000));

  it('get the x after auto exec', async () => {
    const contract = genContract(abi, addr);
    const ret = await contract.methods.x().call('pending');
    logger.debug('\nThe x:\n', ret);
    expect(+ret).to.be.above(1);
  });

  it('get the coinbase after auto exec', async () => {
    const contract = genContract(abi, addr);
    const ret = await contract.methods.coinBase().call('pending');
    logger.debug('\nThe coinBase:\n', ret);
    expect(ret).to.not.equal('0x0000000000000000000000000000000000000000');
  });

  it('should not exec tx', async () => {
    const res = await autoExec();
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt:', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.equal('Reverted.');
  });
});
