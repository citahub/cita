const Nervos = require('@nervos/web3').default;
const config = require('../config');
const log4js = require('log4js');
const Web3 = require('web3');

const web3 = new Web3();
const flag = true;
let nervos;

if (flag) {
  // Use local server
  nervos = Nervos(config.localServer);
} else {
  // Use remote server
  nervos = Nervos(config.remoteServer);
}

const logger = log4js.getLogger();
logger.level = 'debug';
const quota = 9999999;
const blockLimit = 100;
const sender = config.testSender;

const randomInt = () => Math.floor(Math.random() * 100).toString();
const genContract = (abi, addr) => new nervos.eth.Contract(abi, addr);

const getTxReceipt = hash => nervos.listeners.listenToTransactionReceipt(hash);
const getBlockNumber = () => nervos.appchain.getBlockNumber();

const genTxParams = async (_sender = sender) => {
  const current = await getBlockNumber();
  return {
    from: _sender.address,
    privateKey: _sender.privkey,
    nonce: randomInt(),
    quota,
    chainId: 1,
    version: 0,
    validUntilBlock: +current + blockLimit,
    value: '0x0',
    // chainId: nervos.getMetaData().chainId,
  };
};

module.exports = {
  nervos,
  randomInt,
  quota,
  blockLimit,
  genTxParams,
  logger,
  web3,
  genContract,
  getTxReceipt,
  getBlockNumber,
};
