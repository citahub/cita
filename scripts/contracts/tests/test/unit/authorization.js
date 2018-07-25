const authorization = require('../helpers/authorization');
const util = require('../helpers/util');
const config = require('../config');
const chai = require('chai');

const { expect } = chai;

const {
  permissions, resources, superAdmin, rootGroup,
} = config;

const {
  queryPermissions, queryAccounts, checkResource, queryAllAccounts,
} = authorization;

const { logger } = util;

describe('test authorization contract', () => {
  it('should be the build-in authorization: superAdmin has the permission', async () => {
    const res = await queryPermissions(superAdmin.address);
    logger.debug('\nPermissions of superAdmin:\n', res);
    expect(res).to.deep.equal(permissions);
  });

  it('should be the build-in authorization: rootGroup has the permission', async () => {
    const res = await queryPermissions(rootGroup);
    logger.debug('\nPermissions of rootGroup:\n', res);

    for (let i = 0; i < 2; i += 1) { expect(res[i]).to.equal(permissions[i]); }
  });

  it('should be the build-in authorization: account of the permission', async () => {
    const results = permissions.map(p => queryAccounts(p));
    const res = await Promise.all(results);
    logger.debug('\nAccounts:\n', res);
    for (let i = 0; i < res.length; i += 1) {
      if (i > 1) {
        expect(res[i]).to.deep.equal([superAdmin.address]);
      } else {
        expect(res[i]).to.deep.equal([superAdmin.address, rootGroup]);
      }
    }
  });

  it('should check the superAdmin has the resource', async () => {
    const results = [];
    for (let i = 0; i < resources.length; i += 1) {
      for (let j = 1; j < resources[i].length; j += 1) {
        const r = checkResource(
          superAdmin.address,
          resources[i][0],
          resources[i][j],
        );
        results.push(r);
      }
    }
    const res = await Promise.all(results);
    res.map(r => expect(r).to.be.true);
  });

  it('should check the rootGroup has the resource', async () => {
    const results = [];
    for (let i = 0; i < 2; i += 1) {
      for (let j = 1; j < resources[i].length; j += 1) {
        const r = checkResource(
          superAdmin.address,
          resources[i][0],
          resources[i][j],
        );
        results.push(r);
      }
    }
    const res = await Promise.all(results);
    for (let i = 0; i < res.length; i += 1) {
      expect(res[i]).to.be.true;
    }
  });

  it('should check the superAdmin does not have the resource: wrong func', async () => {
    const res = await checkResource(
      superAdmin.address,
      '0xffffffffffffffffffffffffffffffffff020004',
      '0xf036ed57',
    );
    logger.debug('\nResult of check:\n', res);
    expect(res).to.be.false;
  });

  it('should check the superAdmin does not have the resource: wrong cont', async () => {
    const res = await checkResource(
      superAdmin.address,
      '0xffffffffffffffffffffffffffffffffff020005',
      '0xf036ed56',
    );
    logger.debug('\nResult of check:\n', res);
    expect(res).to.be.false;
  });

  it('should have all the accounts', async () => {
    const res = await queryAllAccounts();
    logger.debug('\nAll accounts:\n', res);
    expect(superAdmin.address).to.be.oneOf(res);
  });
});
