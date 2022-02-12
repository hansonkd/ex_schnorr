pub mod atoms;
use rand::{Rng, rngs::OsRng};
use schnorrkel::{Keypair,Signature, SecretKey, PublicKey, signing_context};
use std::io::Write;
use bincode::{serialize, deserialize};
use rustler::Encoder;


pub struct PrivateKeyResource {
    pub key: SecretKey,
}

pub struct PublicKeyResource {
    pub key: PublicKey,
}

#[rustler::nif]
fn sign<'a>(
    env: rustler::Env<'a>,
    private_key: rustler::Term<'a>,
    msg: rustler::Binary,
    context: rustler::Binary,
) -> Result<rustler::Term<'a>, rustler::Error> {
    let key_resource: rustler::ResourceArc<PrivateKeyResource> = private_key.decode()?;
    let key = &key_resource.key.clone().to_keypair();
    let message = msg.as_slice();
    let context = signing_context(context.as_slice());
    let signature: Signature = key.sign(context.bytes(message));

    let bytes = signature.to_bytes();

    let mut bin = match rustler::OwnedBinary::new(bytes.len()) {
        Some(bin) => bin,
        None => panic!("binary term allocation fail"),
    };
    bin.as_mut_slice()
        .write_all(&bytes[..])
        .expect("memory copy of string failed");

    Ok((atoms::ok(), bin.release(env)).encode(env))
}

#[rustler::nif]
fn verify<'a>(
    env: rustler::Env<'a>,
    public_key: rustler::Term<'a>,
    msg: rustler::Binary,
    signature: rustler::Binary,
    context: rustler::Binary
) -> Result<rustler::Term<'a>, rustler::Error> {
    let sig = match deserialize(signature.as_slice()) {
        Ok(sig) => sig,
        Err(e) => return Ok((atoms::error(), atoms::invalid_signature()).encode(env))
    };
    let key_resource: rustler::ResourceArc<PublicKeyResource> = public_key.decode()?;
    let context = signing_context(context.as_slice());
    Ok((atoms::ok(), key_resource.key.verify(context.bytes(msg.as_slice()), &sig).is_ok()).encode(env))
}

#[rustler::nif]
fn keypair<'a>(
    env: rustler::Env<'a>
) -> Result<rustler::Term<'a>, rustler::Error> {

    let keypair: Keypair = Keypair::generate_with(OsRng);
    let pub_key_resource = rustler::ResourceArc::new(PublicKeyResource {
        key: keypair.public,
    });
    let priv_key_resource = rustler::ResourceArc::new(PrivateKeyResource {
        key: keypair.secret.clone(),
    });
    Ok((atoms::ok(), pub_key_resource, priv_key_resource).encode(env))
}

#[rustler::nif]
fn private_from_bytes<'a>(
    env: rustler::Env<'a>,
    private_key: rustler::Binary,
) -> Result<rustler::Term<'a>, rustler::Error> {
    let secret = match deserialize(private_key.as_slice()) {
        Ok(sig) => sig,
        Err(e) => return Ok((atoms::error(), atoms::invalid_key()).encode(env))
    };

    let priv_key_resource = rustler::ResourceArc::new(PrivateKeyResource {
        key: secret,
    });
    Ok((atoms::ok(), priv_key_resource).encode(env))
}

#[rustler::nif]
fn public_from_bytes<'a>(
    env: rustler::Env<'a>,
    public_key: rustler::Binary,
) -> Result<rustler::Term<'a>, rustler::Error> {
    let public = match deserialize(public_key.as_slice()) {
        Ok(sig) => sig,
        Err(e) => return Ok((atoms::error(), atoms::invalid_key()).encode(env))
    };
    let pub_key_resource = rustler::ResourceArc::new(PublicKeyResource {
        key: public,
    });
    Ok((atoms::ok(), pub_key_resource).encode(env))
}

#[rustler::nif]
fn private_to_bytes<'a>(
    env: rustler::Env<'a>,
    private_key: rustler::Term<'a>,
) -> Result<rustler::Term<'a>, rustler::Error> {

    let key_resource: rustler::ResourceArc<PrivateKeyResource> = private_key.decode()?;
    let bytes = serialize(&key_resource.key).unwrap();

    let mut bin = match rustler::OwnedBinary::new(bytes.len()) {
        Some(bin) => bin,
        None => panic!("binary term allocation fail"),
    };
    bin.as_mut_slice()
        .write_all(&bytes[..])
        .expect("memory copy of string failed");

    Ok((atoms::ok(), bin.release(env)).encode(env))
}

#[rustler::nif]
fn public_to_bytes<'a>(
    env: rustler::Env<'a>,
    public_key: rustler::Term<'a>,
) -> Result<rustler::Term<'a>, rustler::Error> {
    let key_resource: rustler::ResourceArc<PublicKeyResource> = public_key.decode()?;
    let bytes = serialize(&key_resource.key).unwrap();

    let mut bin = match rustler::OwnedBinary::new(bytes.len()) {
        Some(bin) => bin,
        None => panic!("binary term allocation fail"),
    };
    bin.as_mut_slice()
        .write_all(&bytes[..])
        .expect("memory copy of string failed");

    Ok((atoms::ok(), bin.release(env)).encode(env))
}



#[rustler::nif]
fn public_from_private<'a>(
    env: rustler::Env<'a>,
    private_key: rustler::Term<'a>,
) -> Result<rustler::Term<'a>, rustler::Error> {
    let key_resource: rustler::ResourceArc<PrivateKeyResource> = private_key.decode()?;
    let resource = rustler::ResourceArc::new(PublicKeyResource {
        key: key_resource.key.clone().to_public(),
    });
    Ok((atoms::ok(), resource).encode(env))
}


fn load(env: rustler::Env, _info: rustler::Term) -> bool {
    rustler::resource!(PublicKeyResource, env);
    rustler::resource!(PrivateKeyResource, env);
    true
}

rustler::init!("Elixir.ExSchnorr", [keypair, sign, verify, public_to_bytes, public_from_bytes, private_from_bytes, private_to_bytes, public_from_private], load = load);
