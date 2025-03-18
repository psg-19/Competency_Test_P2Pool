
use bitcoincore_rpc::{Auth, Client, RpcApi};
use bitcoin::{Amount, Block, OutPoint, Script, Transaction, TxIn, TxOut};
use std::error::Error;
 
use bitcoin::blockdata::locktime::absolute::LockTime;
  
use bitcoin::{ Witness, Sequence};
fn main() -> Result<(), Box<dyn Error>> {
   
    let url="http://127.0.0.1:18443";
   
    let rpc = Client::new(url, Auth::UserPass("alice".to_string(), "password".to_string()))?;
    
    let my_wallet = "Pratush_gupt";

   //loading all wallets to check if wallet already exists
   let all_wallets = rpc.list_wallets()?;
   
   
   // checking if wallet exists then directly load it
    if !all_wallets.contains(&my_wallet.to_string()) {

let wallet_loading=rpc.load_wallet(my_wallet);

        match wallet_loading {
            Ok(_) => println!("Wallet '{}' loaded successfully.", my_wallet),
            Err(e) => {
                println!("Failed to load wallet '{}'", my_wallet);
                println!("wallet_loading problem  {}",e.to_string());

                
                //error in loading wallet so creating a new one
                let create_wallet_result = rpc.create_wallet(
                    my_wallet,
                    Some(false),  
                    Some(false),  
                    Some(""),     
                    Some(false),  
                )?;
                println!("Wallet creation result: {:#?}", create_wallet_result);
            }
        }

    } else {
        println!("Wallet '{}' is already exists.", my_wallet);
    }

    // creating a new address from the wallet.
    let addess_1 = rpc.get_new_address(None, None)?;
    println!("address_1 =>  {:?}", addess_1);

    // mining a block which includes coinbase txn 
    let all_block_hshes = rpc.generate_to_address(1, &addess_1.clone().assume_checked())?;
    let check_block_hsh = all_block_hshes.get(0);
    
    let initial_block_hash;
    match check_block_hsh {
        Some(x)=>initial_block_hash=x,
        None=>{
             
            panic!("No Block was mined, stopped execution !!!!");
        }
        
    }

    
    println!("Mined initial block hash: {}", initial_block_hash);

    //getting the mined block
    let initial_block: Block = rpc.get_block(initial_block_hash)?;
    //extracting the coinbase txn from this block
    let coinbase_tx = initial_block.txdata.first().ok_or("No coinbase transaction found")?;


    println!("Coinbase txn in this block --->{:#?}", coinbase_tx);

    // miining 101  blocks more to mature the coinbase transaction
    //although only 100 blocks required to be mined for the coinbase txn to be matured 
     rpc.generate_to_address(101, &addess_1.clone().assume_checked())?;

     
   
    //===================================================================================
    //creating the first txn to spend the coinbase output
   
    let fee:Amount =Amount::from_sat(1000); // fee in satoshis for tx1
    let coinbase_txn_value  = coinbase_tx.output[0].value;

    // println!("jeeeee {:?}", coinbase_tx.output[0].value);
    // println!("jeeeee {:?}", fee);
    
    if coinbase_txn_value <= fee {
        panic!("Coinbase value is too low to cover fee for tx1");
    }


    let txn1_value = coinbase_txn_value - fee;

    // creating new address for the txn1 recipient
    let recipient1 = rpc.get_new_address(None, None)?;
    println!("recipient1: {:?}", recipient1);

    // txn1 input  
    let txn1_in = TxIn {
        previous_output: OutPoint {
            txid: coinbase_tx.compute_txid(),
            vout: 0,
        },
        script_sig: Script::new().into(),  
        sequence: Sequence(0xFFFFFFFF),      
        witness: Witness::new(),           
    };

    //   txn1 output  
    let txn1_out = TxOut {
        value:  txn1_value,
        script_pubkey: recipient1.assume_checked().script_pubkey(),
    };

    let txn1 = Transaction {
        version: bitcoin::transaction::Version(1),
        lock_time: LockTime::from_height(0).expect("valid height"),
        input: vec![txn1_in],
        output: vec![txn1_out],
    };

    println!("unsigned txn1: {:#?}", txn1);

     //signing txn1 
    // #[derive(Display)]
    let signed_tx1 = rpc.sign_raw_transaction_with_wallet(&txn1, None, None)?;
    if !signed_tx1.complete {
        panic!("Transaction tx1 was not completely signed");
    }
    println!("Signed tx1 : {:?}", signed_tx1);

    // submiting tx1.
    let tx1id = rpc.send_raw_transaction(&signed_tx1.hex)?;
    println!("txn1  txid: {}", tx1id);
 
//===============================txn2 to spend output from txn1========================================


    
    let txn1_output_value = txn1_value; // tx1 has one output with value tx1_value
    if txn1_output_value <= fee {
        panic!("Tx1 output value is too low to cover fee for tx2");
    }
    let txn2_value = txn1_output_value - fee;

    // creating new address for the txn2 recipient.
    let recipient2 = rpc.get_new_address(None, None)?;
    println!("recipient2 ---> {:?}", recipient2);

    //  txn2 input using the txn1 output.
    let txn2_input = TxIn {
        previous_output: OutPoint {
            txid: tx1id,
            vout: 0,
        },

        script_sig: Script::new().into(),  
        sequence: Sequence(0xFFFFFFFF),     
        witness: Witness::new(),  
        
    };

    // txn2 output 
    let txn2_output = TxOut {
        value: txn2_value,
        script_pubkey: recipient2.assume_checked().script_pubkey(),
    };

    let txn2 = Transaction {
        version: bitcoin::transaction::Version(1),
        lock_time: LockTime::from_height(0).expect("valid height"),
        input: vec![txn2_input],
        output: vec![txn2_output],
    };

    println!("unsigned txn2 ---> {:#?}", txn2);

    // signing txn2.
    let signed_tx2 = rpc.sign_raw_transaction_with_wallet(&txn2, None, None)?;
    if !signed_tx2.complete {
       panic!("Transaction txn2 was not completely signed");
    }
    println!("signed txn2 ---> {:?}", signed_tx2);

    // submitting txn2.
    let tx2id = rpc.send_raw_transaction(&signed_tx2.hex)?;
    println!("Transaction tx2 sent with txid: {}", tx2id);

    Ok(())
}
