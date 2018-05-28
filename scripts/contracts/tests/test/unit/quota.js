const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const quota = require('../helpers/quota');

const { logger } = util;
const { describe, it } = mocha;

const {
  getAQL, getDefaultAQL, getBQL, getQuotas, getAccounts, isAdmin,
} = quota;

// =======================

describe('test quota manager constructor', () => {
  it('should have build-in admin', () => {
    const res = isAdmin(quota.admin.address);
    logger.debug('\nthe account is the admin:\n', res);
    assert.equal(res, true);
  });

  it('should have build-in special account', () => {
    const res = getAccounts();
    logger.debug('\nthe special accounts:\n', res);
    assert.equal(res[0], quota.admin.address);
  });

  it('should have build-in quotas of special accounts', () => {
    const res = getQuotas();
    logger.debug('\nthe quotas of the special accounts:\n', res);
    assert.equal(res[0], 1073741824);
  });

  it('should have build-in block quota limit', () => {
    const res = getBQL();
    logger.debug('\nthe block quota limit:\n', res);
    assert.equal(res, 1073741824);
  });

  it('should have build-in default quota limit of account', () => {
    const res = getDefaultAQL();
    logger.debug('\nthe default quota limit of account:\n', res);
    assert.equal(res, 268435456);
  });

  it('should have build-in quota of admin', () => {
    const res = getAQL(quota.admin.address);
    logger.debug('\nthe quota of admin:\n', res);
    assert.equal(res, 1073741824);
  });
});
