const Web3 = require('web3');
const config = require('../config');
const log4js = require('log4js');

const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const logger = log4js.getLogger();
logger.level = 'debug';

const sender = config.testSender;

const randomInt = function random() {
  return Math.floor(Math.random() * 100).toString();
};

const getTxReceipt = function getTxReceipt(res) {
  return new Promise((resolve, reject) => {
    let count = 0;
    const filter = web3.eth.filter('latest', (err) => {
      if (err) reject(err);

      count += 1;

      if (count > 20) {
        filter.stopWatching(() => {});
        reject(err);
      }

      web3.eth.getTransactionReceipt(res.hash, (e, receipt) => {
        if (e) reject(e);

        if (receipt) {
          filter.stopWatching(() => {});
          resolve(receipt);
        }
      });
    });
  });
};

const quota = 9999999;
const blockLimit = 100;

const genTxParams = function genTxParams(_sender = sender) {
  return {
    privkey: _sender.privkey,
    nonce: randomInt(),
    quota,
    validUntilBlock: web3.eth.blockNumber + blockLimit,
    from: sender.address,
    version: 0,
    chainId: web3.eth.getMetaData(0x0).chainId,
  };
};

module.exports = {
  web3,
  randomInt,
  getTxReceipt,
  quota,
  blockLimit,
  genTxParams,
  logger,
};
