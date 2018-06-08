const assert = require('assert');
const mocha = require('mocha');
const util = require('../helpers/util');
const group = require('../helpers/group');

const { web3, logger } = util;
const { it, describe } = mocha;

const {
  queryInfo, queryAccounts, queryParent, inGroup,
} = group;

// =======================

describe('test group contract', () => {
  it('should be the build-in rootGroup', () => {
    const res = queryInfo();
    logger.debug('\nInfo:\n', res);
    assert.equal(res[0].substr(0, 20), web3.toHex('rootGroup'));
    assert.equal(res[1][0], '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523');
  });

  it('should be the build-in accounts', () => {
    const res = queryAccounts();
    logger.debug('\nAccounts:\n', res);
    assert.equal(res[0], '0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523');
  });

  it('should be the build-in parent group', () => {
    const res = queryParent();
    logger.debug('\nParent group:\n', res);
    assert.equal(res, '0x0000000000000000000000000000000000000000');
  });

  it('should in the rootGroup', () => {
    const res = inGroup('0x4b5ae4567ad5d9fb92bc9afd6a657e6fa13a2523');
    logger.debug('\nIs in the group:\n', res);
    assert.equal(res, true);
  });
});
