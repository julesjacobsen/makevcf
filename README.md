# makevcf
Simple CLI for making small VCF files. This will automatically add `INFO` and `FORMAT` lines to the header and correctly
add the sample names and genotypes.

Simple case
---
Something simple, the usual case where you just want a VCF file with a single variant:
```shell
makevcf --out pfeiffer-hg38.vcf --assembly hg38 --variant '10-121496701-T-G|0/1' --format GT --sample manuel
```
Produces:

```text
##fileformat=VCFv4.2
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##assembly=hg38
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	manuel
10	121496701	.	T	G	.	.	.	GT	0/1
```

Compound heterozygous
---
Need two variants for a bi-allelic/compound heterozygous case?
```shell
makevcf --out pfeiffer-hg38.vcf --assembly hg38 --variant '10-121496701-T-G|0/1' --variant '10-121496710-A-C|0/1' --format GT --sample manuel
```
Produces:
```text
##fileformat=VCFv4.2
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##assembly=hg38
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	manuel
10	121496701	.	T	G	.	.	.	GT	0/1
10	121496710	.	A	C	.	.	.	GT	0/1
```

Multi-sample VCF
---
Maybe a multi-sample VCF is needed?
```shell
makevcf --out pfeiffer-quartet-hg19.vcf --assembly hg19 --variant '10-123256215-T-G|0/0:1,0:1:3:0,3,39|0/0:1,0:1:3:0,3,39|1/0:1,0:1:3:0,3,39|0/0:1,0:1:3:0,3,39' --info 'Disease=OMIM:101600;Gene=FGFR2' --format GT:AD:DP:GQ:PL --sample ISDBM322015,ISDBM322016,ISDBM322017,ISDBM322018
```
Produces:
```text
##fileformat=VCFv4.2
##INFO=<ID=Disease,Number=1,Type=String,Description="">
##INFO=<ID=Gene,Number=1,Type=String,Description="">
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##FORMAT=<ID=AD,Number=R,Type=Integer,Description="Read depth for each allele">
##FORMAT=<ID=DP,Number=1,Type=Integer,Description="Read depth">
##FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Conditional genotype quality">
##FORMAT=<ID=PL,Number=G,Type=Integer,Description="Phred-scaled genotype likelihoods rounded to the closest integer">
##assembly=hg19
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	ISDBM322015	ISDBM322016	ISDBM322017	ISDBM322018
10	123256215	.	T	G	.	.	Disease=OMIM%3A101600;Gene=FGFR2	GT:AD:DP:GQ:PL	0/0:1,0:1:3:0,3,39	0/0:1,0:1:3:0,3,39	1/0:1,0:1:3:0,3,39	0/0:1,0:1:3:0,3,39
```

Beast mode
---
Something you'd never want to manually enter:
```shell
makevcf --out test.vcf --assembly GRCh38 --variant '1-12345-AG-T,TC,TTT|0/1:1,4:5:30:81,0,31|0/1:18,19:37:99:505,0,531|0/1:5,6:11:99:177,0,146' --variant '2-23456-TC-A,AG,AAA|0/1:44,39:83:99:763,0,1038|0/1:17,14:31:99:349,0,472|0/1:16,14:30:99:343,0,468' --format GT:AD:DP:GQ:PL --sample ID00001,ID00002,ID00003 --info 'AC=1;AF=0.5;AN=2;BaseQRankSum=1.469;DB;DP=64;Dels=0;FS=5.504;HRun=0;HaplotypeScore=12.8016;MQ0=0;MQ=54.9;MQRankSum=-3.324;QD=7.86;ReadPosRankSum=-0.35;set=variant2' --info 'AC=1;AF=0.5;AN=2;BaseQRankSum=1.557;DB;DP=61;Dels=0;FS=1.021;HRun=1;HaplotypeScore=0;MQ0=2;MQ=46.92;MQRankSum=1.173;QD=14.1;ReadPosRankSum=-0.008;set=variant2'
```
Produces:
```text
##fileformat=VCFv4.2
##INFO=<ID=AC,Number=A,Type=Integer,Description="Allele count in genotypes, for each ALT allele, in the same order as listed">
##INFO=<ID=AF,Number=A,Type=Float,Description="Allele frequency for each ALT allele in the same order as listed (estimated from primary data, not called genotypes)">
##INFO=<ID=AN,Number=1,Type=Integer,Description="Total number of alleles in called genotypes">
##INFO=<ID=BaseQRankSum,Number=1,Type=String,Description="">
##INFO=<ID=DB,Number=0,Type=Flag,Description="dbSNP membership">
##INFO=<ID=DP,Number=1,Type=Integer,Description="Combined depth across samples">
##INFO=<ID=Dels,Number=1,Type=String,Description="">
##INFO=<ID=FS,Number=1,Type=String,Description="">
##INFO=<ID=HRun,Number=1,Type=String,Description="">
##INFO=<ID=HaplotypeScore,Number=1,Type=String,Description="">
##INFO=<ID=MQ,Number=1,Type=Float,Description="RMS mapping quality">
##INFO=<ID=MQ0,Number=1,Type=Integer,Description="Number of MAPQ == 0 reads">
##INFO=<ID=MQRankSum,Number=1,Type=String,Description="">
##INFO=<ID=QD,Number=1,Type=String,Description="">
##INFO=<ID=ReadPosRankSum,Number=1,Type=String,Description="">
##INFO=<ID=set,Number=1,Type=String,Description="">
##FORMAT=<ID=GT,Number=1,Type=String,Description="Genotype">
##FORMAT=<ID=AD,Number=R,Type=Integer,Description="Read depth for each allele">
##FORMAT=<ID=DP,Number=1,Type=Integer,Description="Read depth">
##FORMAT=<ID=GQ,Number=1,Type=Integer,Description="Conditional genotype quality">
##FORMAT=<ID=PL,Number=G,Type=Integer,Description="Phred-scaled genotype likelihoods rounded to the closest integer">
##assembly=GRCh38
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	ID00001	ID00002	ID00003
1	12345	.	AG	T,TC,TTT	.	.	AC=1;AF=0.5;AN=2;BaseQRankSum=1.469;DB;DP=64;Dels=0;FS=5.504;HRun=0;HaplotypeScore=12.8016;MQ0=0;MQ=54.9;MQRankSum=-3.324;QD=7.86;ReadPosRankSum=-0.35;set=variant2	GT:AD:DP:GQ:PL	0/1:1,4:5:30:81,0,31	0/1:18,19:37:99:505,0,531	0/1:5,6:11:99:177,0,146
2	23456	.	TC	A,AG,AAA	.	.	AC=1;AF=0.5;AN=2;BaseQRankSum=1.557;DB;DP=61;Dels=0;FS=1.021;HRun=1;HaplotypeScore=0;MQ0=2;MQ=46.92;MQRankSum=1.173;QD=14.1;ReadPosRankSum=-0.008;set=variant2	GT:AD:DP:GQ:PL	0/1:44,39:83:99:763,0,1038	0/1:17,14:31:99:349,0,472	0/1:16,14:30:99:343,0,468
```