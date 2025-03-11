use anyhow::{anyhow, Result};
use clap::Parser;
use noodles::vcf::header::record::value::map::{Format, Info};
use noodles::vcf::header::record::Value;
use noodles::vcf::variant::io::Write;
use noodles::vcf::{
    self,
    header::{record::value::Map, FileFormat},
    io::Writer,
    Header,
};
use std::collections::BTreeSet;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "makevcf",
    about = "CLI application for generating VCF files",
    version = "0.1.0"
)]
struct Cli {
    /// Output VCF file path
    #[clap(long, required = true)]
    out: PathBuf,

    /// Reference assembly e.g. hg38, hg19, GRCh37, GRCh38, CHM13-T2T
    #[clap(long, required = true)]
    assembly: String,

    /// Variant specifications in format: CHR-POS-REF-ALT|GT:DP|GT:DP ... with the GENOTYPE fields for each sample included with pipe '|' delimiters.
    ///   Multi-allelic sites should use the ',' delimiter for alternate alleles e.g. 1-12345-AG-T,TC,TTT|0/1|1/1|1/2
    #[clap(long, required = true)]
    variant: Vec<String>,

    /// INFO field values. Use repeated values in the same order as the variants if more than one variant is specified. e.g. --variant var1 --variant var2 --info var1_info --info var2_info
    #[clap(long, required = false)]
    info: Vec<String>,

    /// FORMAT fields for genotypes (e.g., GT:DP)
    #[clap(long, required = true, value_delimiter = ':')]
    format: Vec<String>,

    /// Sample names
    #[clap(long, required = true, value_delimiter = ',')]
    sample: Vec<String>,
}

#[derive(Debug)]
struct Variant {
    chromosome: String,
    position: usize,
    reference: String,
    alternate: Vec<String>,
    info: String,
    genotypes: Vec<String>,
}

/// e.g. '1-12345-AG-T:TC:TTT|0/1:50:3,10|1/1:30:7,6|1/2:25:5,5'
fn parse_variant(variant_str: &str, info_option: &Option<&String>) -> Result<Variant> {
    let parts: Vec<&str> = variant_str.split('|').collect();

    if parts.len() < 2 {
        return Err(anyhow!("Invalid variant format. Genotype section required: {}", variant_str));
    }

    let loc_parts: Vec<&str> = parts[0].split('-').collect();
    if loc_parts.len() != 4 {
        return Err(anyhow!("Invalid variant format. Expected format CHR-POS-REF-ALT, but got: {}", parts[0]));
    }

    let chromosome = loc_parts[0].to_string();
    let position = loc_parts[1].parse::<usize>()?;
    let reference = loc_parts[2].to_string();
    let alternate = loc_parts[3].split(",").map(|s| s.to_string()).collect();
    let info = info_option.unwrap_or(&String::from(".")).into();
    let genotypes = parts[1..].iter().map(|s| s.to_string()).collect();

    Ok(Variant {
        chromosome,
        position,
        reference,
        alternate,
        info,
        genotypes,
    })
}

fn build_header(
    assembly: &str,
    format_fields: &[String],
    samples: &[String],
    infos: &[String],
) -> Result<Header> {
    let mut header = Header::builder().set_file_format(FileFormat::new(4, 2));

    // Add reference assembly
    // let contig = Map::<Contig>::new();
    // header = header.add_contig("sq0", contig.clone());
    // header = header.add_contig("1", contig);
    header = header.insert("assembly".parse()?, Value::from(assembly))?;

    // Add FORMAT key fields
    for field in format_fields {
        let format = Map::<Format>::from(field.as_str());
        header = header.add_format(field.as_str(), format);
    }

    // Add INFO key fields
    let info_keys = infos
        .iter()
        .map(|s| s.split(";"))
        .flatten()
        .map(|s| {
            if s.contains("=") {
                let tokens: Vec<&str> = s.split("=").collect();
                tokens[0]
            } else {
                s
            }
        })
        .collect::<BTreeSet<_>>();

    for key in info_keys {
        let info = Map::<Info>::from(key);
        header = header.add_info(key, info)
    }

    // Add samples
    for sample in samples {
        header = header.add_sample_name(sample);
    }

    Ok(header.build())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate assembly
    let valid_assemblies = ["hg38", "hg19", "b37", "b38", "GRCh37", "GRCh38"];
    if !valid_assemblies.contains(&cli.assembly.as_str()) {
        return Err(anyhow!(
            "Invalid assembly: {}. Must be one of: {}",
            cli.assembly,
            valid_assemblies.join(", ")
        ));
    }

    // Build header
    let header = build_header(&cli.assembly, &cli.format, &cli.sample, &cli.info)?;

    // Open output file
    let mut writer = Writer::new(File::create(&cli.out)?);
    writer.write_header(&header)?;

    // Process variants
    for (i, var_str) in cli.variant.iter().enumerate() {
        let variant = parse_variant(var_str, &cli.info.get(i))?;
        println!("{:?}", variant);
        // Ensure we have the right number of genotypes
        if variant.genotypes.len() != cli.sample.len() {
            return Err(anyhow!(
                "Number of genotypes ({}) does not match number of samples ({})",
                variant.genotypes.len(),
                cli.sample.len()
            ));
        }

        // this is all a bit round-about, but it seems easier to just use the Reader to parse a
        // dynamically generated VCF line rather than having to fiddle about with the noodles API.
        let data = format!(
            "{}\t{}\t.\t{}\t{}\t.\t.\t{}\t{}\t{}",
            &variant.chromosome,
            &variant.position,
            &variant.reference,
            &variant.alternate.join(","),
            &variant.info,
            &cli.format.join(":"),
            &variant.genotypes.join("\t"),
        );
        let mut reader = vcf::io::Reader::new(&data.as_bytes()[..]);
        let mut record = vcf::variant::RecordBuf::default();
        reader.read_record_buf(&header, &mut record)?;

        // Write record
        writer.write_variant_record(&header, &record)?;
    }

    println!("VCF file successfully written to: {}", cli.out.display());

    Ok(())
}
