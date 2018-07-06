const util = require('../helpers/util');
const config = require('../config');
const group = require('../helpers/group');
const chai = require('chai');

const { expect } = chai;

const { web3, logger } = util;

const {
  queryInfo, queryAccounts, queryParent, inGroup,
} = group;

// test data
const { address } = config.superAdmin;
const name = web3.utils.utf8ToHex('rootGroup');
const nul = '0x0000000000000000000000000000000000000000';

describe('test group contract', () => {
  it('should be the build-in rootGroup', async () => {
    const res = await queryInfo();
    logger.debug('\nInfo:\n', res);
    expect(res[0]).to.have.string(name);
    expect(address).to.be.oneOf(res[1]);
  });

  it('should be the build-in accounts', async () => {
    const res = await queryAccounts();
    logger.debug('\nAccounts:\n', res);
    expect(address).to.be.oneOf(res);
  });

  it('should be the build-in parent group', async () => {
    const res = await queryParent();
    logger.debug('\nParent group:\n', res);
    expect(res).to.equal(nul);
  });

  it('should in the rootGroup', async () => {
    const res = await inGroup(address);
    logger.debug('\nIs in the group:\n', res);
    expect(res).to.be.true;
  });
});
