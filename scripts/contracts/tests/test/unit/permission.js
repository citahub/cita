const util = require('../helpers/util');
const permission = require('../helpers/permission');
const chai = require('chai');

const { expect } = chai;

const { web3, logger } = util;

const { inPermission, queryInfo } = permission;

describe('test permission contract', () => {
  it('should be the build-in newPermission', async () => {
    const res = await queryInfo();
    logger.debug('\nInfo:\n', res);
    expect(web3.utils.hexToUtf8(res[0])).to.have.string('newPermission');
    expect(res[1]).to.deep.equal(['0xffFffFffFFffFFFFFfFfFFfFFFFfffFFff020004']);
    expect(res[2]).to.deep.equal(['0xfc4a089c']);
  });

  it('test resource in permission', async () => {
    const res = await inPermission(
      '0xffffffffffffffffffffffffffffffffff020004',
      '0xfc4a089c',
    );
    logger.debug('\nThe result:\n', res);
    expect(res).to.equal(true);
  });

  it('test resource not in permission: wrong address', async () => {
    const res = await inPermission(
      '0xffffffffffffffffffffffffffffffffff020005',
      '0xf036ed56',
    );
    logger.debug('\nThe result:\n', res);
    expect(res).to.equal(false);
  });

  it('test resource not in permission: wrong function', async () => {
    const res = await inPermission(
      '0xffffffffffffffffffffffffffffffffff020004',
      '0xf036ed57',
    );
    logger.debug('\nThe result:\n', res);
    expect(res).to.equal(false);
  });

  it('test resource not in permission: all wrong', async () => {
    const res = await inPermission(
      '0xffffffffffffffffffffffffffffffffff020005',
      '0xf036ed57',
    );
    logger.debug('\nThe result:\n', res);
    expect(res).to.equal(false);
  });
});
