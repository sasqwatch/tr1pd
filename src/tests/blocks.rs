// use machine::{SignMachine, VerifyMachine};
use crypto::{self, SignRing};
use crypto::{PublicKey, Signature};
use blocks::prelude::*;

fn bytes2vec(x: &[u8]) -> Vec<u8> {
    let mut vec = Vec::new();
    vec.extend(x);
    vec
}

/*
#[test]
fn init() {
    let (pk, sk) = crypto::gen_keypair();

    let _sm = SignMachine::new(pk.clone(), sk, None);
    let _vm = VerifyMachine::new(pk, None);
}

#[test]
fn simple() {
    let (pk, sk) = crypto::gen_keypair();

    let sm = SignMachine::new(pk.clone(), sk, None);
    let vm = VerifyMachine::new(pk, None);

    let block = sm.unclean_rekey();
    vm.append_block(block).unwrap();
}
*/

    /*
    let pk = crypto::to_pubkey(&[
        4, 247, 115, 241, 237, 193, 65, 43,
        166, 186, 127, 179, 128, 23, 195, 41,
        71, 21, 221, 227, 210, 144, 94, 122,
        208, 170, 204, 78, 89, 197, 99, 217
    ]).unwrap();
    let sk = crypto::to_privkey(&[
        43, 57, 142, 134, 84, 184, 194, 225,
        11, 53, 84, 42, 157, 164, 202, 39,
        235, 27, 7, 37, 85, 144, 93, 223,
        159, 197, 79, 25, 177, 12, 107, 45,
        4, 247, 115, 241, 237, 193, 65, 43,
        166, 186, 127, 179, 128, 23, 195, 41,
        71, 21, 221, 227, 210, 144, 94, 122,
        208, 170, 204, 78, 89, 197, 99, 217
    ]).unwrap();
    */

#[test]
fn fmt_pointer() {
    let pointer = BlockPointer::from_slice(&[
        0x4f, 0x5e, 0x21, 0xd0, 0xa6, 0x56, 0xe8, 0xfd,
        0xe2, 0xb6, 0xd1, 0x0c, 0x4b, 0x2e, 0x9a, 0x96,
        0x27, 0x3c, 0x6b, 0xb6, 0x20, 0x77, 0xee, 0x66,
        0xbe, 0x1d, 0x18, 0x61, 0x1d, 0xd9, 0xab, 0x3e,
    ]).unwrap();

    assert_eq!(format!("{:x}", pointer), *"4f5e21d0a656e8fde2b6d10c4b2e9a96273c6bb62077ee66be1d18611dd9ab3e");

    let pointer2 = BlockPointer::from_hex(&format!("{:x}", pointer)).unwrap();
    assert_eq!(pointer, pointer2);
}

#[test]
fn sha3() {
    let block = Block::from_network(
        // inner
        InitBlock::from_network(
            // prev
            BlockPointer([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]),
            // pubkey
            PublicKey::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap(),
        ).into(),
        // signature
        Signature::from_slice(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        ]).unwrap()
    );

    let expected = BlockPointer([
        0xbf, 0x0a, 0x13, 0xf3, 0x38, 0x42, 0xff, 0xd1,
        0x75, 0xe8, 0xe1, 0x37, 0x34, 0xd7, 0x8c, 0xbe,
        0x85, 0x86, 0xd3, 0x0f, 0xfe, 0x63, 0xcf, 0x78,
        0x33, 0xaf, 0x39, 0x50, 0xa8, 0x4b, 0x6f, 0x57,
    ]);

    assert_eq!(block.sha3(), expected);
}

#[test]
fn init() {
    let (pk, sk) = crypto::gen_keypair();
    let mut sm = SignRing::new(pk, sk);

    let pointer = None;
    let _block = Block::init(pointer.into(), &mut sm).unwrap();
}

#[test]
fn init_bytes() {
    let expected = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // previous block
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x00, // op
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // pubkey
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];

    let block = Block::from_network(
        // inner
        InitBlock::from_network(
            // prev
            BlockPointer([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]),
            // pubkey
            PublicKey::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap(),
        ).into(),
        // signature
        Signature::from_slice(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        ]).unwrap()
    );

    let expected = bytes2vec(&expected);
    assert_eq!(expected, block.encode());
}

#[test]
fn rekey() {
    let (pk, sk) = crypto::gen_keypair();
    let mut sm = SignRing::new(pk, sk);

    let pointer = None;
    let dummy = Block::init(pointer.into(), &mut sm).unwrap();

    let _block = Block::rekey(dummy.prev().clone(), &mut sm).unwrap();
}

#[test]
fn rekey_bytes() {
    let expected = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // previous block
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, // op
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // pubkey
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];

    let block = Block::from_network(
        // inner
        RekeyBlock::from_network(
            // prev
            BlockPointer([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]),
            // pubkey
            PublicKey::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap(),
            // signature
            Signature::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap()
        ).into(),
        // signature
        Signature::from_slice(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        ]).unwrap()
    );

    let expected = bytes2vec(&expected);
    assert_eq!(expected, block.encode());
}

#[test]
fn alert() {
    let (pk, sk) = crypto::gen_keypair();
    let mut sm = SignRing::new(pk, sk);

    let pointer = None;
    let dummy = Block::init(pointer.into(), &mut sm).unwrap();

    let _block = Block::alert(dummy.prev().clone(), &mut sm, "ohai".as_bytes().to_vec()).unwrap();
}

#[test]
fn alert_bytes() {
    let expected = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // previous block
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x02, // op
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // pubkey
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x00, 0x04, // length
        0x6f, 0x68, 0x61, 0x69, // payload
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];

    let block = Block::from_network(
        // inner
        AlertBlock::from_network(
            // prev
            BlockPointer([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]),
            // pubkey
            PublicKey::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap(),
            // bytes
            "ohai".as_bytes().to_vec(),
            // signature
            Signature::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap()
        ).into(),
        // signature
        Signature::from_slice(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        ]).unwrap()
    );

    let expected = bytes2vec(&expected);
    assert_eq!(expected, block.encode());
}

#[test]
fn info() {
    let (pk, sk) = crypto::gen_keypair();
    let mut sm = SignRing::new(pk, sk);

    let pointer = None;
    let dummy = Block::init(pointer.into(), &mut sm).unwrap();

    let _block = Block::info(dummy.prev().clone(), &mut sm, "ohai".as_bytes().to_vec()).unwrap();
}

#[test]
fn info_bytes() {
    let expected = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // previous block
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x03, // op
        0x00, 0x04, // length
        0x6f, 0x68, 0x61, 0x69, // payload
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, // signature
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
    ];

    let block = Block::from_network(
        // inner
        InfoBlock::from_network(
            // prev
            BlockPointer([
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]),
            // bytes
            "ohai".as_bytes().to_vec(),
            // signature
            Signature::from_slice(&[
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            ]).unwrap()
        ).into(),
        // signature
        Signature::from_slice(&[
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
        ]).unwrap()
    );

    let expected = bytes2vec(&expected);
    assert_eq!(expected, block.encode());
}

use engine::Engine;
use storage::{MemoryStorage, BlockStorage};

#[test]
fn test_small_block() {
    let (pk, sk) = crypto::gen_keypair();
    let ring = SignRing::new(pk, sk);
    let storage = MemoryStorage::new().to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();

    let written = engine.info([0; 25].to_vec()).unwrap(); // 25B
    let storage = engine.storage();
    let head = storage.get_head().unwrap();
    let info = storage.get(&head).unwrap();

    assert_eq!(info, written);
}

#[test]
fn test_large_block() {
    let (pk, sk) = crypto::gen_keypair();
    let ring = SignRing::new(pk, sk);
    let storage = MemoryStorage::new().to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();

    let written = engine.info([0; 65535].to_vec()).unwrap(); // 65KiB, max block size
    let storage = engine.storage();
    let head = storage.get_head().unwrap();
    let info = storage.get(&head).unwrap();

    assert_eq!(info, written);
}

#[test]
fn test_too_large_block() {
    let (pk, sk) = crypto::gen_keypair();
    let ring = SignRing::new(pk, sk);
    let storage = MemoryStorage::new().to_engine();
    let mut engine = Engine::start(storage, ring).unwrap();

    let err = engine.info([0; 1024*70].to_vec()).err().unwrap(); // 70KiB
    match *err.kind() {
        ::engine::errors::ErrorKind::Blocks(::blocks::ErrorKind::BlockTooLarge) => (),
        _ => panic!("not BlockTooLarge error"),
    };
}
