/* jshint esversion: 6 */
/* jshint expr: true */

const Web3 = require('web3');
const config = require('../config');
const web3 = new Web3(new Web3.providers.HttpProvider(config.localServer));
// Use remote server
// const web3 = new Web3(new Web3.providers.HttpProvider(config.remoteServer));

const randomInt = function () {
    return Math.floor(Math.random() * 100).toString();
};

const getTxReceipt = function (res) {
    return new Promise((resolve, reject) => { 
        let count = 0;
        const filter = web3.eth.filter('latest', err => {

            if (err) reject(err);
            
            count++;

            if (count > 20) {
                filter.stopWatching(function() {});
                reject(err);
            }

            web3.eth.getTransactionReceipt(res.hash, function(err, receipt) {

                if (err) reject(err);

                if (receipt) {
                    filter.stopWatching(function() {});
                    resolve(receipt);
                }
            });
        });
    });
};

const quota = 9999999;
const blockLimit = 100;

module.exports = {
    web3,
    randomInt,
    getTxReceipt,
    quota,
    blockLimit
};
