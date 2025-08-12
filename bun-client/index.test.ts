import {test, expect} from 'bun:test'
import * as borsh from 'borsh';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from '@solana/web3.js';
import { ACC_SIZE, CounterAccount, schema } from './types';

let adminKeyPair: Keypair;
let counterAccountKeyPair: Keypair;

test('check data account creation and value check', async () => { 
    adminKeyPair = Keypair.generate();
    counterAccountKeyPair = new Keypair();
    
    const connection = new Connection('http://127.0.0.1:8899');
    const txn = await connection.requestAirdrop(adminKeyPair.publicKey, 5 * LAMPORTS_PER_SOL);
    await connection.confirmTransaction(txn);
    const lamports = await connection.getMinimumBalanceForRentExemption(ACC_SIZE)

    const createCounterTxn = SystemProgram.createAccount({
        fromPubkey: adminKeyPair.publicKey,
        newAccountPubkey: counterAccountKeyPair.publicKey,
        programId: new PublicKey('FW4dGKo9QPtn9P1Lm4QMZL76tFVgPH7GZHiMnVuPgLc9'),
        space: ACC_SIZE,
        lamports 
    })
    
    const transaction = new Transaction();
    transaction.add(createCounterTxn);

    const txHash = await connection.sendTransaction(transaction, [adminKeyPair, counterAccountKeyPair])
    await connection.confirmTransaction(txHash);

    const acc = await connection.getAccountInfo(counterAccountKeyPair.publicKey);

    if(!acc) {
        throw new Error("Counter account does not found");
    }

    const result = borsh.deserialize(schema, acc.data) as CounterAccount;
    console.log(result);
    expect(result.count).toBe(0);
    expect(1).toBe(1);
})

test('check for function working', async () => {

  expect(1).toBe(1);
})
