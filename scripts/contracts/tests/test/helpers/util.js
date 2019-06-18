const CITASDK = require('@cryptape/cita-sdk').default;
const log4js = require('log4js');
const config = require('../config');

const flag = true;
let citaSDK;

if (flag) {
  // Use local server
  citaSDK = CITASDK(config.localServer);
} else {
  // Use remote server
  citaSDK = CITASDK(config.remoteServer);
}

const logger = log4js.getLogger();
logger.level = 'debug';
const quota = 9999999;
const blockLimit = 100;
const sender = config.testSender;

const randomInt = () => Math.floor(Math.random() * 100).toString();
const genContract = (abi, addr) => new citaSDK.eth.Contract(abi, addr);

const getTxReceipt = citaSDK.listeners.listenToTransactionReceipt;
const { getBlockNumber, getMetaData } = citaSDK.base;

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
  citaSDK,
  randomInt,
  quota,
  blockLimit,
  genTxParams,
  logger,
  genContract,
  getTxReceipt,
  getBlockNumber,
  getMetaData,
};
