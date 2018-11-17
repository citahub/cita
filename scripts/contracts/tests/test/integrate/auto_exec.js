const chai = require('chai');
const auto = require('../helpers/auto_exec');
const util = require('../helpers/util');
const config = require('../config');

const { expect } = chai;
const {
  nervos, logger, genTxParams, genContract, getTxReceipt,
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
const bin = '608060405234801561001057600080fd5b5060cf8061001f6000396000f3006080604052600436106049576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680630c55699c14604e578063844cbc43146076575b600080fd5b348015605957600080fd5b506060608a565b6040518082815260200191505060405180910390f35b348015608157600080fd5b5060886090565b005b60005481565b60008081548092919060010191905055505600a165627a7a72305820d5b2c5380ae2f0103722d0da7d082e8f342e2a017de0fd63f11c48cfc4a0b0140029';
const abi = [{
  constant: true, inputs: [], name: 'x', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  constant: false, inputs: [], name: 'autoExec', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}];

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
