use crate::cli_constants;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use zokrates_common::constants;
use zokrates_common::helpers::*;
#[cfg(feature = "ark")]
use zokrates_core::proof_system::ark::Ark;
#[cfg(feature = "bellman")]
use zokrates_core::proof_system::bellman::Bellman;
#[cfg(feature = "libsnark")]
use zokrates_core::proof_system::libsnark::Libsnark;
#[cfg(any(feature = "bellman", feature = "ark", feature = "libsnark"))]
use zokrates_core::proof_system::*;
use zokrates_field::{Bls12_377Field, Bls12_381Field, Bn128Field, Bw6_761Field, Field};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("verify")
        .about("Verifies a given proof with the given verification key")
        .arg(
            Arg::with_name("proof-path")
                .short("j")
                .long("proof-path")
                .help("Path of the JSON proof file")
                .value_name("FILE")
                .takes_value(true)
                .required(false)
                .default_value(cli_constants::JSON_PROOF_PATH),
        )
        .arg(
            Arg::with_name("verification-key-path")
                .short("v")
                .long("verification-key-path")
                .help("Path of the generated verification key file")
                .value_name("FILE")
                .takes_value(true)
                .required(false)
                .default_value(cli_constants::VERIFICATION_KEY_DEFAULT_PATH),
        )
        .arg(
            Arg::with_name("backend")
                .short("b")
                .long("backend")
                .help("Backend to use")
                .takes_value(true)
                .required(false)
                .possible_values(cli_constants::BACKENDS)
                .default_value(constants::BELLMAN),
        )
}

pub fn exec(sub_matches: &ArgMatches) -> Result<(), String> {
    let vk_path = Path::new(sub_matches.value_of("verification-key-path").unwrap());
    let vk_file = File::open(&vk_path)
        .map_err(|why| format!("Could not open {}: {}", vk_path.display(), why))?;

    // deserialize vk to JSON
    let vk_reader = BufReader::new(vk_file);
    let vk: serde_json::Value = serde_json::from_reader(vk_reader)
        .map_err(|why| format!("Could not deserialize verification key: {}", why))?;

    let proof_path = Path::new(sub_matches.value_of("proof-path").unwrap());
    let proof_file = File::open(&proof_path)
        .map_err(|why| format!("Could not open {}: {}", proof_path.display(), why))?;

    // deserialize proof to JSON
    let proof_reader = BufReader::new(proof_file);
    let proof: serde_json::Value = serde_json::from_reader(proof_reader)
        .map_err(|why| format!("Could not deserialize proof: {}", why))?;

    // extract curve and scheme parameters from both
    let proof_curve = proof
        .get("curve")
        .ok_or("Field `curve` not found in proof".to_string())?
        .as_str()
        .ok_or("`curve` should be a string".to_string())?;
    let proof_scheme = proof
        .get("scheme")
        .ok_or("Field `scheme` not found in proof".to_string())?
        .as_str()
        .ok_or("`scheme` should be a string".to_string())?;
    let vk_curve = vk
        .get("curve")
        .ok_or("Field `curve` not found in verification key".to_string())?
        .as_str()
        .ok_or("`curve` should be a string".to_string())?;
    let vk_scheme = vk
        .get("scheme")
        .ok_or("Field `scheme` not found in verification key".to_string())?
        .as_str()
        .ok_or("`scheme` should be a string".to_string())?;

    if proof_curve != vk_curve {
        return Err(format!(
            "Expected the curve of the proof and the verification to be equal, found {} != {}",
            proof_curve, vk_curve
        ));
    }

    if proof_curve != vk_curve {
        return Err(format!(
            "Expected the scheme of the proof and the verification to be equal, found {} != {}",
            proof_scheme, vk_scheme
        ));
    }

    // determine parameters based on that
    let parameters = Parameters::try_from((
        sub_matches.value_of("backend").unwrap(),
        proof_curve,
        proof_scheme,
    ))?;

    match parameters {
        #[cfg(feature = "bellman")]
        Parameters(BackendParameter::Bellman, CurveParameter::Bn128, SchemeParameter::G16) => {
            cli_verify::<Bn128Field, G16, Bellman>(vk, proof)
        }
        #[cfg(feature = "bellman")]
        Parameters(BackendParameter::Bellman, CurveParameter::Bls12_381, SchemeParameter::G16) => {
            cli_verify::<Bls12_381Field, G16, Bellman>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bn128, SchemeParameter::G16) => {
            cli_verify::<Bn128Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_381, SchemeParameter::G16) => {
            cli_verify::<Bls12_381Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_377, SchemeParameter::G16) => {
            cli_verify::<Bls12_377Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bw6_761, SchemeParameter::G16) => {
            cli_verify::<Bw6_761Field, G16, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bn128, SchemeParameter::GM17) => {
            cli_verify::<Bn128Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_381, SchemeParameter::GM17) => {
            cli_verify::<Bls12_381Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_377, SchemeParameter::GM17) => {
            cli_verify::<Bls12_377Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bw6_761, SchemeParameter::GM17) => {
            cli_verify::<Bw6_761Field, GM17, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bn128, SchemeParameter::MARLIN) => {
            cli_verify::<Bn128Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_381, SchemeParameter::MARLIN) => {
            cli_verify::<Bls12_381Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bls12_377, SchemeParameter::MARLIN) => {
            cli_verify::<Bls12_377Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "ark")]
        Parameters(BackendParameter::Ark, CurveParameter::Bw6_761, SchemeParameter::MARLIN) => {
            cli_verify::<Bw6_761Field, Marlin, Ark>(vk, proof)
        }
        #[cfg(feature = "libsnark")]
        Parameters(BackendParameter::Libsnark, CurveParameter::Bn128, SchemeParameter::GM17) => {
            cli_verify::<Bn128Field, GM17, Libsnark>(vk, proof)
        }
        #[cfg(feature = "libsnark")]
        Parameters(BackendParameter::Libsnark, CurveParameter::Bn128, SchemeParameter::PGHR13) => {
            cli_verify::<Bn128Field, PGHR13, Libsnark>(vk, proof)
        }
        _ => unreachable!(),
    }
}

fn cli_verify<T: Field, S: Scheme<T>, B: Backend<T, S>>(
    vk: serde_json::Value,
    proof: serde_json::Value,
) -> Result<(), String> {
    // convert the JSON vk and proof to the correct types
    let vk = serde_json::from_value(vk)
        .map_err(|why| format!("Could not deserialize verification key: {}", why))?;
    let proof: Proof<T, S> = serde_json::from_value(proof)
        .map_err(|why| format!("Could not deserialize proof: {}", why))?;

    println!("Performing verification...");
    println!(
        "{}",
        match B::verify(vk, proof) {
            true => "PASSED",
            false => "FAILED",
        }
    );

    Ok(())
}
