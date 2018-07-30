// / NOTICE Should use `--contract_arguments "SysConfig.checkQuota=true"`

const chai = require('chai');
const util = require('../helpers/util');
const quota = require('../helpers/quota');

const { expect } = chai;
const {
  getTxReceipt, logger, nervos, genTxParams,
} = util;
const { setDefaultAQL, getDefaultAQL } = quota;

// temp
let hash;

// test data
const valueLess = '9999990';
const valueMore = '19999999';

describe('test quota not enough error', () => {
  it('should send a tx: setDefaultAQL ', async () => {
    const res = await setDefaultAQL(valueLess);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt: setDefaultAQL', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('should wait a block', done => setTimeout(done, 10000));

  it('should have new default account quota limit', async () => {
    const res = await getDefaultAQL();
    logger.debug('\nthe default account quota limit:\n', res);
    expect(res).to.equal(valueLess);
  });

  it('should send a tx with bigger quota and get error msg', async () => {
    const param = await genTxParams();
    // TODO should receive an error
    nervos.appchain.sendTransaction(param).catch((e) => {
      logger.log('\nerror:\n', e);
      expect(e).to.equal('QuotaNotEnough');
    });
  });

  it('should send a tx: setDefaultAQL ', async () => {
    const res = await setDefaultAQL(valueMore);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt: setDefaultAQL', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('should wait a block', done => setTimeout(done, 10000));

  it('should have new default account quota limit', async () => {
    const res = await getDefaultAQL();
    logger.debug('\nthe default account quota limit:\n', res);
    expect(res).to.equal(valueMore);
  });

  it('should send a tx with bigger quota', async () => {
    const param = await genTxParams();
    const res = await nervos.appchain.sendTransaction(param);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
  });
});
