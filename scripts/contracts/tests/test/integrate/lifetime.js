const chai = require('chai');
const util = require('../helpers/util');
const config = require('../config');

const { expect } = chai;
const {
  citaSDK, logger, genTxParams, genContract, getTxReceipt,
} = util;

const { superAdmin } = config;

// tmp
let addr;
let param;
let hash;
let balance;
const value = '0x100000';

// test data
const bin = '6080604052336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506000600260006101000a81548160ff0219169083151502179055506103aa8061006e6000396000f300608060405260043610610083576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff168063054f7d9c1461008557806341c0e1b5146100b457806360fe47b1146100cb57806362a5af3b146100f85780636a28f0001461010f5780636d4ce63c146101265780638da5cb5b14610151575b005b34801561009157600080fd5b5061009a6101a8565b604051808215151515815260200191505060405180910390f35b3480156100c057600080fd5b506100c96101bb565b005b3480156100d757600080fd5b506100f660048036038101908080359060200190929190505050610280565b005b34801561010457600080fd5b5061010d610315565b005b34801561011b57600080fd5b50610124610332565b005b34801561013257600080fd5b5061013b61034f565b6040518082815260200191505060405180910390f35b34801561015d57600080fd5b50610166610359565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b600260009054906101000a900460ff1681565b60001515600260009054906101000a900460ff161515141515610246576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260188152602001807f546869732066756e6374696f6e2069732066726f7a656e2e000000000000000081525060200191505060405180910390fd5b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16ff5b60001515600260009054906101000a900460ff16151514151561030b576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260188152602001807f546869732066756e6374696f6e2069732066726f7a656e2e000000000000000081525060200191505060405180910390fd5b8060018190555050565b6001600260006101000a81548160ff021916908315150217905550565b6000600260006101000a81548160ff021916908315150217905550565b6000600154905090565b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815600a165627a7a72305820c3bf27393a60762b7e0ea2853f01849fccbc0f1c065fad2f2db0db1a005e36ee0029';

const abi = [{
  constant: true, inputs: [], name: 'frozen', outputs: [{ name: '', type: 'bool' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  constant: false, inputs: [], name: 'kill', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}, {
  constant: false, inputs: [{ name: 'x', type: 'uint256' }], name: 'set', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}, {
  constant: false, inputs: [], name: 'freeze', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}, {
  constant: false, inputs: [], name: 'unfreeze', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}, {
  constant: true, inputs: [], name: 'get', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  constant: true, inputs: [], name: 'owner', outputs: [{ name: '', type: 'address' }], payable: false, stateMutability: 'view', type: 'function',
}, {
  inputs: [], payable: true, stateMutability: 'payable', type: 'constructor',
}, { payable: true, stateMutability: 'payable', type: 'fallback' }];

describe('\n\nDeploy a contract\n\n', () => {
  it('should send a tx: deploy_contract', async () => {
    param = await genTxParams(superAdmin);
    const res = await citaSDK.base.deploy(
      bin,
      { ...param, value },
    );
    logger.debug('\nDeploy a contract:\n', res.contractAddress);
    addr = res.contractAddress;
  });
});

describe('\n\ntest lifetime of contract\n\n', () => {
  it('get the balance of superAdmin and contract', async () => {
    balance = await citaSDK.base.getBalance(superAdmin.address, 'pending');
    const res = await citaSDK.base.getBalance(addr, 'pending');
    logger.debug('\nThe balance of admin: %s,\nand contract: %s\n', balance, res);
    expect(+res).to.equal(+value);
  });

  it('should send a suicide tx', async () => {
    const contract = genContract(abi, addr);
    const res = await contract.methods.kill().send(param);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt:', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('get the balance of superAdmin and contract', async () => {
    const resAdmin = await citaSDK.base.getBalance(superAdmin.address, 'pending');
    const resCont = await citaSDK.base.getBalance(addr, 'pending');
    logger.debug('\nThe balance of admin: %s\nThe contract: %s', resAdmin, resCont);
    expect(+resCont).to.equal(0);
    // Not equal balance+value: cause the tx fee.
    expect(+resAdmin).to.be.above(+balance);
  });
});
