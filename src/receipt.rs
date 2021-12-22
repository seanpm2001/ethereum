use crate::util::enveloped;
use crate::Log;
use alloc::vec::Vec;
use ethereum_types::{Bloom, H256, U256};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use rlp_derive::{RlpDecodable, RlpEncodable};

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FrontierReceiptData {
	pub state_root: H256,
	pub used_gas: U256,
	pub logs_bloom: Bloom,
	pub logs: Vec<Log>,
}

#[derive(Clone, Debug, PartialEq, Eq, RlpEncodable, RlpDecodable)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EIP658ReceiptData {
	pub status_code: u8,
	pub used_gas: U256,
	pub logs_bloom: Bloom,
	pub logs: Vec<Log>,
}

pub type EIP2930ReceiptData = EIP658ReceiptData;

pub type EIP1559ReceiptData = EIP658ReceiptData;

pub type ReceiptV0 = FrontierReceiptData;

pub type ReceiptV1 = EIP658ReceiptData;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReceiptV2 {
	/// Legacy receipt type
	Legacy(EIP658ReceiptData),
	/// EIP-2930 receipt type
	EIP2930(EIP2930ReceiptData),
}

impl Encodable for ReceiptV2 {
	fn rlp_append(&self, s: &mut RlpStream) {
		match self {
			Self::Legacy(r) => r.rlp_append(s),
			Self::EIP2930(r) => enveloped(1, r, s),
		}
	}
}

impl Decodable for ReceiptV2 {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let slice = rlp.data()?;

		let first = *slice.get(0).ok_or(DecoderError::Custom("empty slice"))?;

		if rlp.is_list() {
			return Ok(Self::Legacy(Decodable::decode(rlp)?));
		}

		let s = slice
			.get(1..)
			.ok_or(DecoderError::Custom("no receipt body"))?;

		if first == 0x01 {
			return rlp::decode(s).map(Self::EIP2930);
		}

		Err(DecoderError::Custom("invalid receipt type"))
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
	feature = "with-codec",
	derive(codec::Encode, codec::Decode, scale_info::TypeInfo)
)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ReceiptV3 {
	/// Legacy receipt type
	Legacy(EIP658ReceiptData),
	/// EIP-2930 receipt type
	EIP2930(EIP2930ReceiptData),
	/// EIP-1559 receipt type
	EIP1559(EIP1559ReceiptData),
}

impl Encodable for ReceiptV3 {
	fn rlp_append(&self, s: &mut RlpStream) {
		match self {
			Self::Legacy(r) => r.rlp_append(s),
			Self::EIP2930(r) => enveloped(1, r, s),
			Self::EIP1559(r) => enveloped(2, r, s),
		}
	}
}

impl Decodable for ReceiptV3 {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		let slice = rlp.data()?;

		let first = *slice.get(0).ok_or(DecoderError::Custom("empty slice"))?;

		if rlp.is_list() {
			return Ok(Self::Legacy(Decodable::decode(rlp)?));
		}

		let s = slice
			.get(1..)
			.ok_or(DecoderError::Custom("no receipt body"))?;

		if first == 0x01 {
			return rlp::decode(s).map(Self::EIP2930);
		}

		if first == 0x02 {
			return rlp::decode(s).map(Self::EIP1559);
		}

		Err(DecoderError::Custom("invalid receipt type"))
	}
}
