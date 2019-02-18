const Appchain = require('@appchain/base').default;
const config = require('../config');
const log4js = require('log4js');

const flag = true;
let appchain;

if (flag) {
  // Use local server
  appchain = Appchain(config.localServer);
} else {
  // Use remote server
  appchain = Appchain(config.remoteServer);
}

const logger = log4js.getLogger();
logger.level = 'debug';
const quota = 9999999;
const blockLimit = 100;
const sender = config.testSender;

const randomInt = () => Math.floor(Math.random() * 100).toString();
const genContract = (abi, addr) => new appchain.eth.Contract(abi, addr);

const getTxReceipt = appchain.listeners.listenToTransactionReceipt;
const getBlockNumber = appchain.base.getBlockNumber;
const getMetaData = appchain.base.getMetaData;

const genTxParams = async (_sender = sender) => {
  const current = await getBlockNumber();
  const metaData = await getMetaData();
  return {
    from: _sender.address,
    privateKey: _sender.privkey,
    nonce: randomInt(),
    quota,
    chainId: metaData.chainIdV1,
    version: metaData.version,
    validUntilBlock: +current + blockLimit,
    value: '0x0',
  };
};

module.exports = {
  appchain,
  randomInt,
  quota,
  blockLimit,
  genTxParams,
  logger,
  genContract,
  getTxReceipt,
  getBlockNumber,
};
