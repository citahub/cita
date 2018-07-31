const util = require('../helpers/util');
const admin = require('../helpers/admin');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;
const {
  logger, getTxReceipt,
} = util;
const {
  isAdmin, update,
} = admin;
const { testAddr, superAdmin } = config;

// temo
let hash;

// test data
const addr = testAddr[0];

describe('\n\ntest admin\n\n', () => {
  it('should have built-in superAdmin', async () => {
    const res = await isAdmin(superAdmin.address);
    logger.debug('\nthe account is an superAdmin:\n', res);
    expect(res).to.be.true;
  });

  it('should not be superAdmin: testAddr', async () => {
    const res = await isAdmin(addr);
    logger.debug('\nthe account is an superAdmin:\n', res);
    expect(res).to.be.false;
  });

  it('should send a tx: update', async () => {
    const res = await update(addr);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt: update', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
  });

  it('should update superAdmin', async () => {
    const res = await isAdmin(addr);
    logger.debug('\nthe account is an superAdmin:\n', res);
    expect(res).to.be.true;
  });
});
