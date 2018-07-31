const util = require('../helpers/util');
const quota = require('../helpers/quota');
const chai = require('chai');
const config = require('../config');

const { expect } = chai;
const { logger } = util;
const { superAdmin } = config;

const {
  getAQL, getDefaultAQL, getBQL, getQuotas, getAccounts,
} = quota;

// test data TODO as a file
const admin = superAdmin.address;
const BQL = '1073741824';
const defaultAQL = '268435456';
const AQL = '1073741824';

describe('test quota manager constructor', () => {
  it('should have build-in special account', async () => {
    const res = await getAccounts();
    logger.debug('\nthe special accounts:\n', res);
    expect(res[0]).to.equal(admin);
  });

  it('should have build-in quotas of special accounts', async () => {
    const res = await getQuotas();
    logger.debug('\nthe quotas of the special accounts:\n', res);
    expect(res[0]).to.equal(AQL);
  });

  it('should have build-in block quota limit', async () => {
    const res = await getBQL();
    logger.debug('\nthe block quota limit:\n', res);
    expect(res).to.equal(BQL);
  });

  it('should have build-in default quota limit of account', async () => {
    const res = await getDefaultAQL();
    logger.debug('\nthe default quota limit of account:\n', res);
    expect(res).to.equal(defaultAQL);
  });

  it('should have build-in quota of admin', async () => {
    const res = await getAQL(admin);
    logger.debug('\nthe quota of admin:\n', res);
    expect(res).to.equal(AQL);
  });
});
