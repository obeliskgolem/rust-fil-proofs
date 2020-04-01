use clap::{values_t, App, Arg};
use log::info;
use rand::rngs::OsRng;

use filecoin_proofs::constants::*;
use filecoin_proofs::parameters::{
    election_post_public_params, public_params, window_post_public_params,
    winning_post_public_params,
};
use filecoin_proofs::types::*;
use filecoin_proofs::PoStType;
use std::collections::HashSet;
use storage_proofs::compound_proof::CompoundProof;
use storage_proofs::parameter_cache::CacheableParameters;
use storage_proofs::porep::stacked::{StackedCompound, StackedDrg};
use storage_proofs::post::election::{ElectionPoSt, ElectionPoStCircuit, ElectionPoStCompound};
use storage_proofs::post::fallback::{FallbackPoSt, FallbackPoStCircuit, FallbackPoStCompound};

const PUBLISHED_SECTOR_SIZES: [u64; 4] = [
    SECTOR_SIZE_2_KIB,
    SECTOR_SIZE_8_MIB,
    SECTOR_SIZE_512_MIB,
    SECTOR_SIZE_32_GIB,
];

fn cache_porep_params(porep_config: PoRepConfig) {
    let n = u64::from(PaddedBytesAmount::from(porep_config));
    info!(
        "begin PoRep parameter-cache check/populate routine for {}-byte sectors",
        n
    );

    let public_params = public_params(
        PaddedBytesAmount::from(porep_config),
        usize::from(PoRepProofPartitions::from(porep_config)),
    )
    .unwrap();

    {
        let circuit = <StackedCompound<DefaultTreeHasher, DefaultPieceHasher> as CompoundProof<
            StackedDrg<DefaultTreeHasher, DefaultPieceHasher>,
            _,
        >>::blank_circuit(&public_params);
        let _ = StackedCompound::<DefaultTreeHasher, DefaultPieceHasher>::get_param_metadata(
            circuit,
            &public_params,
        );
    }
    {
        let circuit = <StackedCompound<DefaultTreeHasher, DefaultPieceHasher> as CompoundProof<
            StackedDrg<DefaultTreeHasher, DefaultPieceHasher>,
            _,
        >>::blank_circuit(&public_params);
        StackedCompound::<DefaultTreeHasher, DefaultPieceHasher>::get_groth_params(
            Some(&mut OsRng),
            circuit,
            &public_params,
        )
        .expect("failed to get groth params");
    }
    {
        let circuit = <StackedCompound<DefaultTreeHasher, DefaultPieceHasher> as CompoundProof<
            StackedDrg<DefaultTreeHasher, DefaultPieceHasher>,
            _,
        >>::blank_circuit(&public_params);

        StackedCompound::<DefaultTreeHasher, DefaultPieceHasher>::get_verifying_key(
            Some(&mut OsRng),
            circuit,
            &public_params,
        )
        .expect("failed to get verifying key");
    }
}

fn cache_election_post_params(post_config: &PoStConfig) {
    let n = u64::from(post_config.padded_sector_size());
    info!(
        "begin Election PoSt parameter-cache check/populate routine for {}-byte sectors",
        n
    );

    let post_public_params = election_post_public_params(post_config).unwrap();

    {
        let post_circuit: ElectionPoStCircuit<DefaultTreeHasher> =
            <ElectionPoStCompound<DefaultTreeHasher> as CompoundProof<
                ElectionPoSt<DefaultTreeHasher>,
                ElectionPoStCircuit<DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);
        let _ = <ElectionPoStCompound<DefaultTreeHasher>>::get_param_metadata(
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get metadata");
    }
    {
        let post_circuit: ElectionPoStCircuit<DefaultTreeHasher> =
            <ElectionPoStCompound<DefaultTreeHasher> as CompoundProof<
                ElectionPoSt<DefaultTreeHasher>,
                ElectionPoStCircuit<DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);
        <ElectionPoStCompound<DefaultTreeHasher>>::get_groth_params(
            Some(&mut OsRng),
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get groth params");
    }
    {
        let post_circuit: ElectionPoStCircuit<DefaultTreeHasher> =
            <ElectionPoStCompound<DefaultTreeHasher> as CompoundProof<
                ElectionPoSt<DefaultTreeHasher>,
                ElectionPoStCircuit<DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);

        <ElectionPoStCompound<DefaultTreeHasher>>::get_verifying_key(
            Some(&mut OsRng),
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get verifying key");
    }
}

fn cache_winning_post_params(post_config: &PoStConfig) {
    let n = u64::from(post_config.padded_sector_size());
    info!(
        "begin Winning PoSt parameter-cache check/populate routine for {}-byte sectors",
        n
    );

    let post_public_params = winning_post_public_params(post_config).unwrap();

    {
        let post_circuit: FallbackPoStCircuit<Bls12, DefaultTreeHasher> =
            <FallbackPoStCompound<DefaultTreeHasher> as CompoundProof<
                Bls12,
                FallbackPoSt<DefaultTreeHasher>,
                FallbackPoStCircuit<Bls12, DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);
        let _ = <FallbackPoStCompound<DefaultTreeHasher>>::get_param_metadata(
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get metadata");
    }
    {
        let post_circuit: FallbackPoStCircuit<Bls12, DefaultTreeHasher> =
            <FallbackPoStCompound<DefaultTreeHasher> as CompoundProof<
                Bls12,
                FallbackPoSt<DefaultTreeHasher>,
                FallbackPoStCircuit<Bls12, DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);
        <FallbackPoStCompound<DefaultTreeHasher>>::get_groth_params(
            Some(&mut OsRng),
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get groth params");
    }
    {
        let post_circuit: FallbackPoStCircuit<Bls12, DefaultTreeHasher> =
            <FallbackPoStCompound<DefaultTreeHasher> as CompoundProof<
                Bls12,
                FallbackPoSt<DefaultTreeHasher>,
                FallbackPoStCircuit<Bls12, DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);

        <FallbackPoStCompound<DefaultTreeHasher>>::get_verifying_key(
            Some(&mut OsRng),
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get verifying key");
    }
}

fn cache_window_post_params(post_config: &PoStConfig) {
    let n = u64::from(post_config.padded_sector_size());
    info!(
        "begin Window PoSt parameter-cache check/populate routine for {}-byte sectors",
        n
    );

    let post_public_params = window_post_public_params(post_config).unwrap();

    {
        let post_circuit: FallbackPoStCircuit<Bls12, DefaultTreeHasher> =
            <FallbackPoStCompound<DefaultTreeHasher> as CompoundProof<
                Bls12,
                FallbackPoSt<DefaultTreeHasher>,
                FallbackPoStCircuit<Bls12, DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);
        let _ = <FallbackPoStCompound<DefaultTreeHasher>>::get_param_metadata(
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get metadata");
    }
    {
        let post_circuit: FallbackPoStCircuit<Bls12, DefaultTreeHasher> =
            <FallbackPoStCompound<DefaultTreeHasher> as CompoundProof<
                Bls12,
                FallbackPoSt<DefaultTreeHasher>,
                FallbackPoStCircuit<Bls12, DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);
        <FallbackPoStCompound<DefaultTreeHasher>>::get_groth_params(
            Some(&mut OsRng),
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get groth params");
    }
    {
        let post_circuit: FallbackPoStCircuit<Bls12, DefaultTreeHasher> =
            <FallbackPoStCompound<DefaultTreeHasher> as CompoundProof<
                Bls12,
                FallbackPoSt<DefaultTreeHasher>,
                FallbackPoStCircuit<Bls12, DefaultTreeHasher>,
            >>::blank_circuit(&post_public_params);

        <FallbackPoStCompound<DefaultTreeHasher>>::get_verifying_key(
            Some(&mut OsRng),
            post_circuit,
            &post_public_params,
        )
        .expect("failed to get verifying key");
    }
}

// Run this from the command-line to pre-generate the groth parameters used by the API.
pub fn main() {
    fil_logger::init();

    let matches = App::new("paramcache")
        .version("0.1")
        .about("Generate and persist Groth parameters and verifying keys")
        .arg(
            Arg::with_name("params-for-sector-sizes")
                .short("z")
                .long("params-for-sector-sizes")
                .conflicts_with("all")
                .require_delimiter(true)
                .value_delimiter(",")
                .multiple(true)
                .help("A comma-separated list of sector sizes, in bytes, for which Groth parameters will be generated")
        )
        .arg(
            Arg::with_name("only-post")
                .long("only-post")
                .help("Only generate parameters for post")
        )
        .get_matches();

    let sizes: HashSet<u64> = if matches.is_present("params-for-sector-sizes") {
        values_t!(matches.values_of("params-for-sector-sizes"), u64)
            .unwrap()
            .into_iter()
            .collect()
    } else {
        PUBLISHED_SECTOR_SIZES.iter().cloned().collect()
    };

    let only_post = matches.is_present("only-post");

    for sector_size in sizes {
        cache_election_post_params(&PoStConfig {
            sector_size: SectorSize(sector_size),
            challenge_count: ELECTION_POST_CHALLENGE_COUNT,
            sector_count: 1,
            typ: PoStType::Election,
            priority: true,
        });

        cache_winning_post_params(&PoStConfig {
            sector_size: SectorSize(sector_size),
            challenge_count: WINNING_POST_CHALLENGE_COUNT,
            sector_count: WINNING_POST_SECTOR_COUNT,
            typ: PoStType::Winning,
            priority: true,
        });

        cache_window_post_params(&PoStConfig {
            sector_size: SectorSize(sector_size),
            challenge_count: WINDOW_POST_CHALLENGE_COUNT,
            sector_count: *WINDOW_POST_SECTOR_COUNT
                .read()
                .unwrap()
                .get(&sector_size)
                .unwrap(),
            typ: PoStType::Window,
            priority: true,
        });

        if !only_post {
            cache_porep_params(PoRepConfig {
                sector_size: SectorSize(sector_size),
                partitions: PoRepProofPartitions(
                    *POREP_PARTITIONS
                        .read()
                        .unwrap()
                        .get(&sector_size)
                        .expect("missing sector size"),
                ),
            });
        }
    }
}
