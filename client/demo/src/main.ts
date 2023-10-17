import { Client, TransferParams } from '../../src';

let client = await Client.new('http://localhost:18340', key => true);

console.log(client);

let transfer: TransferParams = {
  private_key: 'APrivateKey1zkp31UnRUkKRWCFeKQDf1N9CV9uZxuGtpnrQPRbKRtaFrzW',
  recipient: 'aleo17ha7zdps004mphrg9den6n73fn6rvsartqn49qmxrs3m37egnggs8kkfhj',
  amount: 100000000,
  function: 'public',
};

let resp = await client.transfer(transfer);

console.log(resp);
