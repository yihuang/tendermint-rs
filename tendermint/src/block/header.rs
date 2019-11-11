//! Block headers
use crate::merkle::simple_hash_from_byte_slices;
use crate::{account, amino_types, block, chain, lite, Hash, Time};
use prost::Message;
use {
    crate::serializers,
    serde::{Deserialize, Serialize},
};

/// Block `Header` values contain metadata about the block and about the
/// consensus, as well as commitments to the data in the current block, the
/// previous block, and the results returned by the application.
///
/// <https://github.com/tendermint/tendermint/blob/master/docs/spec/blockchain/blockchain.md#header>
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Header {
    /// Header version
    pub version: Version,

    /// Chain ID
    pub chain_id: chain::Id,

    /// Current block height
    pub height: block::Height,

    /// Current timestamp
    pub time: Time,

    /// Number of transactions in block
    #[serde(
        serialize_with = "serializers::serialize_u64",
        deserialize_with = "serializers::parse_u64"
    )]
    pub num_txs: u64,

    /// Total number of transactions
    #[serde(
        serialize_with = "serializers::serialize_u64",
        deserialize_with = "serializers::parse_u64"
    )]
    pub total_txs: u64,

    /// Previous block info
    pub last_block_id: block::Id,

    /// Commit from validators from the last block
    pub last_commit_hash: Hash,

    /// Merkle root of transaction hashes
    pub data_hash: Hash,

    /// Validators for the current block
    pub validators_hash: Hash,

    /// Validators for the next block
    pub next_validators_hash: Hash,

    /// Consensus params for the current block
    pub consensus_hash: Hash,

    /// State after txs from the previous block
    pub app_hash: Hash,

    /// Root hash of all results from the txs from the previous block
    pub last_results_hash: Hash,

    /// Hash of evidence included in the block
    pub evidence_hash: Hash,

    /// Original proposer of the block
    pub proposer_address: account::Id,
}

impl lite::Header for Header {
    fn height(&self) -> block::Height {
        self.height
    }

    fn bft_time(&self) -> Time {
        self.time
    }

    fn validators_hash(&self) -> Hash {
        self.validators_hash
    }

    fn next_validators_hash(&self) -> Hash {
        self.next_validators_hash
    }

    fn hash(&self) -> Hash {
        let mut version_enc = vec![];
        // TODO: if there is an encoding problem this will
        // panic (as the golang code would):
        // https://github.com/tendermint/tendermint/blob/134fe2896275bb926b49743c1e25493f6b24cc31/types/block.go#L393
        // https://github.com/tendermint/tendermint/blob/134fe2896275bb926b49743c1e25493f6b24cc31/types/encoding_helper.go#L9:6
        // Instead, handle errors gracefully here.
        amino_types::ConsensusVersion::from(&self.version)
            .encode(&mut version_enc)
            .unwrap();
        let height_enc = encode_varint(self.height.value());
        let mut time_enc = vec![];
        amino_types::TimeMsg::from(self.time)
            .encode(&mut time_enc)
            .unwrap();
        let chain_id_bytes = self.chain_id.as_bytes();
        let chain_id_enc = bytes_enc(&chain_id_bytes);
        let num_tx_enc = encode_varint(self.num_txs);
        let total_tx_enc = encode_varint(self.total_txs);
        let mut last_block_id_enc = vec![];
        amino_types::BlockId::from(&self.last_block_id)
            .encode(&mut last_block_id_enc)
            .unwrap();
        let last_commit_hash_enc = encode_hash(self.last_commit_hash);
        let data_hash_enc = encode_hash(self.data_hash);
        let validator_hash_enc = encode_hash(self.validators_hash);
        let next_validator_hash_enc = encode_hash(self.next_validators_hash);
        let consensus_hash_enc = encode_hash(self.consensus_hash);
        let app_hash_enc = encode_hash(self.app_hash);
        let last_result_hash_enc = encode_hash(self.last_results_hash);
        let evidence_hash_enc = encode_hash(self.evidence_hash);
        let proposer_address_bytes = self.proposer_address.as_bytes();
        let proposer_address_enc = bytes_enc(&proposer_address_bytes);

        let mut byteslices: Vec<&[u8]> = vec![];
        byteslices.push(version_enc.as_slice());
        byteslices.push(chain_id_enc.as_slice());
        byteslices.push(height_enc.as_slice());
        byteslices.push(time_enc.as_slice());
        byteslices.push(num_tx_enc.as_slice());
        byteslices.push(total_tx_enc.as_slice());
        byteslices.push(last_block_id_enc.as_slice());
        byteslices.push(last_commit_hash_enc.as_slice());
        byteslices.push(data_hash_enc.as_slice());
        byteslices.push(validator_hash_enc.as_slice());
        byteslices.push(next_validator_hash_enc.as_slice());
        byteslices.push(consensus_hash_enc.as_slice());
        byteslices.push(app_hash_enc.as_slice());
        byteslices.push(last_result_hash_enc.as_slice());
        byteslices.push(evidence_hash_enc.as_slice());
        byteslices.push(proposer_address_enc.as_slice());

        Hash::Sha256(simple_hash_from_byte_slices(byteslices.as_slice()))
    }
}

/// `Version` contains the protocol version for the blockchain and the
/// application.
///
/// <https://github.com/tendermint/tendermint/blob/master/docs/spec/blockchain/blockchain.md#version>
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Version {
    /// Block version
    #[serde(
        serialize_with = "serializers::serialize_u64",
        deserialize_with = "serializers::parse_u64"
    )]
    pub block: u64,

    /// App version
    #[serde(
        serialize_with = "serializers::serialize_u64",
        deserialize_with = "serializers::parse_u64"
    )]
    pub app: u64,
}

fn bytes_enc(bytes: &[u8]) -> Vec<u8> {
    let mut chain_id_enc = vec![];
    prost_amino::encode_length_delimiter(bytes.len(), &mut chain_id_enc).unwrap();
    chain_id_enc.append(&mut bytes.to_vec());
    chain_id_enc
}

fn encode_hash(hash: Hash) -> Vec<u8> {
    let mut hash_enc = vec![];
    if let Some(last_commit_hash_bytes) = hash.as_bytes() {
        hash_enc = bytes_enc(last_commit_hash_bytes);
    }
    hash_enc
}

fn encode_varint(val: u64) -> Vec<u8> {
    let mut val_enc = vec![];
    prost_amino::encoding::encode_varint(val, &mut val_enc);
    val_enc
}
