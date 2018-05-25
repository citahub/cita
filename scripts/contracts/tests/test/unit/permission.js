const mocha = require('mocha');
const assert = require('assert');
const util = require('../helpers/util');
const permission = require('../helpers/permission');

const { web3, logger } = util;

const { inPermission, queryInfo } = permission;

const { describe, it } = mocha;

// =======================

describe('test permission contract', () => {
  it('should be the build-in newPermission', () => {
    const res = queryInfo();
    logger.debug('\nInfo:\n', res);
    assert.equal(res[0].substr(0, 28), web3.toHex('newPermission'));
    assert.equal(res[1], '0x00000000000000000000000000000000013241b2');
    assert.equal(res[2], '0xfc4a089c');
  });

  it('test resource in permission', () => {
    const res = inPermission(
      '0x00000000000000000000000000000000013241b2',
      '0xfc4a089c',
    );
    logger.debug('\nThe result:\n', res);
    assert.equal(res, true);
  });

  it('test resource not in permission: wrong address', () => {
    const res = inPermission(
      '0x00000000000000000000000000000000013241b3',
      '0xf036ed56',
    );
    logger.debug('\nThe result:\n', res);
    assert.equal(res, false);
  });

  it('test resource not in permission: wrong function', () => {
    const res = inPermission(
      '0x00000000000000000000000000000000013241b2',
      '0xf036ed57',
    );
    logger.debug('\nThe result:\n', res);
    assert.equal(res, false);
  });

  it('test resource not in permission: all wrong', () => {
    const res = inPermission(
      '0x00000000000000000000000000000000013241b3',
      '0xf036ed57',
    );
    logger.debug('\nThe result:\n', res);
    assert.equal(res, false);
  });
});
