const chai = require('chai');
const util = require('../helpers/util');
const versionManager = require('../helpers/version_manager');

const { expect } = chai;
const {
  logger, getTxReceipt, getMetaData,
} = util;
const {
  setProtocolVersion, getProtocolVersion, getVersion,
} = versionManager;

// temp
let hash;
let version;

describe('\n\ntest version manager\n\n', () => {
  before('should have version', async () => {
    const metaData = await getMetaData();
    ({ version } = metaData);
    logger.debug('\nthe version of metaData is:\n', version);
  });

  it('should get the protocol version', async () => {
    const res = await getProtocolVersion();
    logger.debug('\nthe version is:\n', res);
    expect(+res).to.be.equal(version);
  });

  it('should send a tx: setProtocolVersion', async () => {
    const res = await setProtocolVersion(version + 1);
    logger.debug('\nSend tx ok:\n', JSON.stringify(res));
    expect(res.status).to.equal('OK');
    ({ hash } = res);
  });

  it('should get receipt: setProtocolVersion', async () => {
    const res = await getTxReceipt(hash);
    logger.debug('\nget receipt:\n', res);
    expect(res.errorMessage).to.be.null;
    version += 1;
  });

  it('should get the protocol version', async () => {
    const res = await getProtocolVersion();
    logger.debug('\nthe version is:\n', res);
    expect(+res).to.be.equal(version);
  });

  // old interface
  it('should get the version', async () => {
    const res = await getVersion();
    logger.debug('\nthe version is:\n', res);
    expect(+res).to.be.equal(version);
  });
});
