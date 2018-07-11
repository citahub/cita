const mocha = require('mocha');
const assert = require('assert');
const authorization = require('../helpers/authorization');
const util = require('../helpers/util');
const config = require('../config');

const { superAdmin } = config.contract.authorization;
const { permissions, resources } = config;

const {
  queryPermissions, queryAccounts, checkResource, queryAllAccounts,
} = authorization;

const { logger } = util;

const { describe, it } = mocha;

const rootGroup = '0xffffffffffffffffffffffffffffffffff020009';
const len = permissions.length;

// =======================

describe('test authorization contract', () => {
  it('should be the build-in authorization: superAdmin has the permission', () => {
    const res = queryPermissions(superAdmin.address);
    logger.debug('\nPermissions of superAdmin:\n', res);

    for (let i = 0; i < len; i += 1) { assert.equal(res[i], permissions[i]); }
  });

  it('should be the build-in authorization: rootGroup has the permission', () => {
    const res = queryPermissions(rootGroup);
    logger.debug('\nPermissions of rootGroup:\n', res);

    for (let i = 0; i < 2; i += 1) { assert.equal(res[i], permissions[i]); }
  });

  it('should be the build-in authorization: account of the permission', () => {
    for (let i = 2; i < len; i += 1) {
      const res = queryAccounts(permissions[i]);
      logger.debug('\nAccount of permissions:\n', res);
      assert.equal(res, superAdmin.address);
    }
    for (let i = 0; i < 2; i += 1) {
      const res = queryAccounts(permissions[i]);
      logger.debug('\nAccount of permissions:\n', res);
      assert.equal(res[0], superAdmin.address);
      assert.equal(res[1], rootGroup);
    }
  });

  it('should check the superAdmin has the resource', () => {
    for (let i = 0; i < resources.length; i += 1) {
      for (let j = 1; j < resources[i].length; j += 1) {
        const res = checkResource(
          superAdmin.address,
          resources[i][0],
          resources[i][j],
        );
        logger.debug('\nResult of check:(%i,%j)\n', i, j, res);
        assert.equal(res, true);
      }
    }
  });

  it('should check the rootGroup has the resource', () => {
    for (let i = 0; i < 2; i += 1) {
      for (let j = 1; j < resources[i].length; j += 1) {
        const res = checkResource(
          superAdmin.address,
          resources[i][0],
          resources[i][j],
        );
        logger.debug('\nResult of check:(%i,%j)\n', i, j, res);
        assert.equal(res, true);
      }
    }
  });

  it('should check the superAdmin does not have the resource: wrong func', () => {
    const res = checkResource(
      superAdmin.address,
      '0xffffffffffffffffffffffffffffffffff020004',
      '0xf036ed57',
    );
    logger.debug('\nResult of check:\n', res);
    assert.equal(res, false);
  });

  it('should check the superAdmin does not have the resource: wrong cont', () => {
    const res = checkResource(
      superAdmin.address,
      '0xffffffffffffffffffffffffffffffffff020005',
      '0xf036ed56',
    );
    logger.debug('\nResult of check:\n', res);
    assert.equal(res, false);
  });

  it('should have all the accounts', () => {
    const res = queryAllAccounts();
    logger.debug('\nAll accounts:\n', res);
    assert.equal(res[0], superAdmin.address);
  });
});
