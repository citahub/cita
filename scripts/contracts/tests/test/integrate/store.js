const chai = require('chai');
const util = require('../helpers/util');
const config = require('../config');

const { expect } = chai;
const { superAdmin } = config;

const {
  logger, appchain, genTxParams, getTxReceipt,
} = util;

// tmp
let hash;
let param;
let content;

// test data
const msg = appchain.utils.utf8ToHex('This is a test');
const store = 'ffffffffffffffffffffffffffffffffff010000';

describe('test store data', () => {
  it('should send a tx with data', async () => {
    param = await genTxParams(superAdmin);
    const res = await appchain.base.sendTransaction({
      ...param,
      to: store,
      data: msg,
    });
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt:', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('should get tx content', async () => {
    const res = await appchain.base.getTransaction(hash);
    logger.debug('\nTransaction:\n', res);
    expect(res.hash).to.equal(hash);
    ({ content } = res);
  });

  it('should equal test msg', async () => {
    const res = await appchain.base.unsigner(content);
    logger.debug('\nunsigner transaction content:\n', res);
    expect(res.transaction.data).to.equal(msg);
  });
});
