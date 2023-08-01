use bitcoin::BlockHash;
use bitcoin::ScriptBuf;
use bitcoin::secp256k1::PublicKey;
use bitcoin::hash_types::Txid;
use bitcoin::blockdata::script::Builder;
use bitcoin::taproot::LeafVersion;
use bitcoin::taproot::NodeInfo;
use bitcoin::taproot::TapTree;
use bitcoincore_rpc::Client as RpcClient;
use bitcoincore_rpc::Error;
use bitcoincore_rpc::Auth;
use bitcoin::consensus::encode::deserialize;
use bitcoincore_rpc::RpcApi;
use bitcoin::blockdata::script as txscript;
use bitcoin::blockdata::opcodes;



// Implement all functionnalities for Write/Read

const PROTOCOL_ID: &[u8] = &[0x72, 0x6f, 0x6c, 0x6c];

// Sample data and keys for testing.
// bob key pair is used for signing reveal tx
// internal key pair is used for tweaking
const BOB_PRIVATE_KEY: &str = "5JoQtsKQuH8hC9MyvfJAqo6qmKLm8ePYNucs7tPu2YxG12trzBt";
const INTERNAL_PRIVATE_KEY: &str = "5JGgKfRy6vEcWBpLJV5FXUfMGNXzvdWzQHUM1rVLEUJfvZUSwvS";

// chunk_slice splits input slice into max chunk_size length slices
fn chunk_slice(slice: &[u8], chunk_size: usize) -> Vec<&[u8]> {
    let mut chunks = Vec::new();
    let mut i = 0;
    while i < slice.len() {
        let end = i + chunk_size;

        // necessary check to avoid slicing beyond
        // slice capacity
        let end = if end > slice.len() {
            slice.len()
        } else {
            end
        };

        chunks.push(&slice[i..end]);
        i = end;
    }

    chunks
}

// create_taproot_address returns an address committing to a Taproot script with
// a single leaf containing the spend path with the script:
// <embedded data> OP_DROP <pubkey> OP_CHECKSIG
// TODO
fn create_taproot_address(embedded_data: &[u8]) -> Result<String, String> {
    // Step 1: Decode bobPrivateKey as WIF

    // Step 2: Get the corresponding public key from the WIF private key

    // Step 3: Build the Taproot script with a single leaf

    // Step 4: Get the corresponding internal public key from internalPrivateKey

    // Step 5: Build the Taproot script tree

    // Step 6: Generate the Taproot output key

    // Step 7: Generate the Bech32m address

    // Step 8: Return the generated Taproot address

    Ok("Nothing for now".to_string())
}

// pay_to_taproot_script creates a pk script for a pay-to-taproot output key.
// TODO
pub fn pay_to_taproot_script(taproot_key: &PublicKey) -> Result<Vec<u8>, String> {
    let builder = Builder::new();

    // OP_1 is equivalent to OP_TRUE in Bitcoin Script.
    builder.clone().push_opcode(bitcoin::blockdata::opcodes::OP_TRUE);

    builder.clone().push_slice(&taproot_key.serialize());

    let script = builder.into_script();
    let script_bytes = script.as_bytes();

    Ok(script_bytes.to_vec())
}

// Relayer is a bitcoin client wrapper which provides reader and writer methods
// to write binary blobs to the blockchain.
struct Relayer {
    client: RpcClient,
}

impl Relayer {
    // NewRelayer creates a new Relayer instance with the provided Config.
    //TO TEST
    fn new_relayer(config: &Config) -> Result<Self, Error> {
        // Set up the connection to the bitcoin RPC server.
        let auth = Auth::UserPass(config.user.clone(), config.pass.clone());
        let client = RpcClient::new(&config.host, auth)?;

        Ok(Relayer { client })
    }

    // close shuts down the client.
    fn close(&self) {
        //TODO
    }

    // commitTx commits an output to the given taproot address, such that the
    // output is only spendable by posting the embedded data on chain, as part of
    // the script satisfying the tapscript spend path that commits to the data. It
    // returns the hash of the commit transaction and error, if any.
    // fn commit_tx(&self, addr: &str) -> Result<Txid, Error> {
    //     //TODO
    // }

    // revealTx spends the output from the commit transaction and as part of the
    // script satisfying the tapscript spend path, posts the embedded data on
    // chain. It returns the hash of the reveal transaction and error, if any.  
    // fn reveal_tx(&self, embedded_data: &[u8], commit_hash: &Txid) -> Result<Txid, Error> {
    //     //TODO
    // }


    // fn ReadTransaction(client: &RpcClient, hash: &Txid) -> Result<Option<Vec<u8>>, Error> {
    //     let raw_tx = client.get_raw_transaction(hash, None)?;
    
    //     if let Ok(tx) = deserialize(&raw_tx) {  //TODO: find a way to deserialize
    //         if let Some(witness) = tx.input[0].witness.get(1) {
    //             if let Some(push_data) = ExtractPushData(0, witness) {
    //                 // Skip PROTOCOL_ID
    //                 if push_data.starts_with(PROTOCOL_ID) {
    //                     return Ok(Some(push_data[PROTOCOL_ID.len()..].to_vec()));
    //                 }
    //             }
    //         }
    //     }
    
    //     Ok(None)
    // }

    // fn Read(&self, height: u64) -> Result<Vec<Vec<u8>>, Box<dyn core::fmt::Debug>> {
    //     let hash = self.client.get_block_hash(height as u64)?;
    //     let block = self.client.get_block(&BlockHash::from(hash))?;
    //     let mut data = Vec::new();

    //     for tx in block.txdata.iter() {
    //         if let Some(witness) = tx.input[0].witness.nth(1) { //Verify that this is the right way to get the witness
    //             if let Some(push_data) = ExtractPushData(0, witness) {
    //                 // Skip PROTOCOL_ID
    //                 if push_data.starts_with(PROTOCOL_ID) {
    //                     data.push(push_data[PROTOCOL_ID.len()..].to_vec());
    //                 }
    //             }
    //         }
    //     }
    //     Ok(data)
    // }

    // fn Write(&self, data: &[u8]) -> Result<Txid, Box<dyn std::error::Error>> {
    //     let mut data_with_protocol_id = PROTOCOL_ID.to_vec();
    //     data_with_protocol_id.extend_from_slice(data);
    
    //     let address = create_taproot_address(&data_with_protocol_id)?;
    //     let commit_hash = self.commit_tx(&address)?;
    //     let reveal_hash = self.reveal_tx(data, &commit_hash)?;
    
    //     Ok(reveal_hash)
    // }
    
}

struct Config {
    host: String,
    user: String,
    pass: String,
    http_post_mode: bool,
    disable_tls: bool,
}

impl Config {
    // Constructor to create a new Config instance
    fn new(host: String, user: String, pass: String, http_post_mode: bool, disable_tls: bool) -> Self {
        Config {
            host,
            user,
            pass,
            http_post_mode,
            disable_tls,
        }
    }
}
#[derive(Default)]
struct TemplateMatch {
    expect_push_data: bool,
    max_push_datas: usize,
    opcode: u8,
    extracted_data: Vec<u8>,
}

fn extract_push_data(version: u8, pk_script: Vec<u8>) -> Option<Vec<u8>> {
    
    let template = [
        TemplateMatch { opcode: opcodes::OP_FALSE.to_u8(), ..Default::default() },
        TemplateMatch { opcode: opcodes::all::OP_IF.to_u8(), ..Default::default() },
        TemplateMatch { expect_push_data: true, max_push_datas: 10, ..Default::default() },
        TemplateMatch { opcode: opcodes::all::OP_ENDIF.to_u8(), ..Default::default() },
        TemplateMatch { expect_push_data: true, max_push_datas: 1, ..Default::default() },
        TemplateMatch { opcode: opcodes::all::OP_CHECKSIG.to_u8(), ..Default::default() },
    ];

    let mut template_offset = 0;

    let node_info = NodeInfo::new_leaf_with_ver(ScriptBuf::from_bytes(pk_script), LeafVersion::from_consensus(version).unwrap());
    let tap_tree = TapTree::try_from(node_info).unwrap();
    
    let mut tokenizer = TapTree::script_leaves(&tap_tree);

    while let Some(op) = tokenizer.next() {

        if template_offset >= template.len() {
            return None;
        }

        let tpl_entry = &template[template_offset];
        
        //To be reviewed on testing
        if !tpl_entry.expect_push_data && op.script().first_opcode().unwrap().to_u8() != tpl_entry.opcode {
            return None;
        }

        template_offset += 1;
    }

    Some(template[2].extracted_data.clone())
}
