const chai = require('chai');
const util = require('../helpers/util');
const config = require('../config');

const { expect } = chai;
const {
  appchain, logger, genTxParams,
} = util;

const abiArray = [{
  constant: false, inputs: [{ name: '_value', type: 'uint256' }], name: 'set', outputs: [], payable: false, stateMutability: 'nonpayable', type: 'function',
}, {
  constant: true, inputs: [], name: 'get', outputs: [{ name: '', type: 'uint256' }], payable: false, stateMutability: 'view', type: 'function',
}];

const abi = JSON.parse('[{"constant":false,"inputs":[{"name":"_value","type":"uint256"}],"name":"set","outputs":[],"payable":false,"stateMutability":"nonpayable","type":"function"},{"constant":true,"inputs":[],"name":"get","outputs":[{"name":"","type":"uint256"}],"payable":false,"stateMutability":"view","type":"function"}]');
const { testBin } = config;
const { superAdmin } = config;

let addr;
let param;

describe('\n\ntest store/get abi\n\n', () => {
  it('should send a tx: deploy_contract', async () => {
    param = await genTxParams(superAdmin);
    const res = await appchain.base.deploy(
      testBin,
      param,
    );
    logger.debug('\nDeploy a contract:\n', res.contractAddress);
    addr = res.contractAddress;
  });

  it('should send a tx: store abi', async () => {
    param = await genTxParams(superAdmin);
    const res = await appchain.base.storeAbi(
      addr,
      abi,
      param,
    );
    logger.debug('\nStore abi:\n', res);
  });

  it('should get the abi', async () => {
    const res = await appchain.base.getAbi(addr, 'pending');
    logger.debug('\nabi of test:\n', res);
    expect(res).to.deep.equal(abiArray);
  });
});
