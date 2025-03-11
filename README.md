# makevcf
Simple CLI for making small VCF files

Something simple, the usual case where you just want a VCF file with a single variant:
```shell
makevcf --out pfeiffer-hg38.vcf --assembly hg38 --variant '10-121496701-T-G|0/1' --format GT --sample manuel
```

Need two variants for a bi-allelic/compound heterozygous case:
```shell
makevcf --out pfeiffer-hg38.vcf --assembly hg38 --variant '10-121496701-T-G|0/1' --variant '10-121496710-A-C|0/1' --format GT --sample manuel
```

maybe a multi-sample VCF is needed?
```shell
#CHROM  POS     ID      REF     ALT     QUAL    FILTER  INFO    FORMAT  ISDBM322015     ISDBM322016     ISDBM322017     ISDBM322018
# 10      123256215       .       T       G       100     PASS    Disease=OMIM:101600;Gene=FGFR2       GT:AD:DP:GQ:PL  0/0:1,0:1:3:0,3,39      0/0:1,0:1:3:0,3,39      1/0:1,0:1:3:0,3,39  0/0:1,0:1:3:0,3,39
makevcf --out pfeiffer-quartet-hg19.vcf --assembly hg19 --variant '10-123256215-T-G|0/0:1,0:1:3:0,3,39|0/0:1,0:1:3:0,3,39|1/0:1,0:1:3:0,3,39|0/0:1,0:1:3:0,3,39' --info 'Disease=OMIM:101600;Gene=FGFR2' --format GT:AD:DP:GQ:PL --sample ISDBM322015,ISDBM322016,ISDBM322017,ISDBM322018
```


Something you'd never want to manually enter:

```shell
makevcf --out test.vcf --assembly GRCh38 --variant '1-12345-AG-T,TC,TTT|0/1:1,4:5:30:81,0,31|0/1:18,19:37:99:505,0,531|0/1:5,6:11:99:177,0,146' --variant '2-23456-TC-A,AG,AAA|0/1:44,39:83:99:763,0,1038|0/1:17,14:31:99:349,0,472|0/1:16,14:30:99:343,0,468' --format GT:AD:DP:GQ:PL --sample ID00001,ID00002,ID00003 --info 'AC=1;AF=0.5;AN=2;BaseQRankSum=1.469;DB;DP=64;Dels=0;FS=5.504;HRun=0;HaplotypeScore=12.8016;MQ0=0;MQ=54.9;MQRankSum=-3.324;QD=7.86;ReadPosRankSum=-0.35;set=variant2' --info 'AC=1;AF=0.5;AN=2;BaseQRankSum=1.557;DB;DP=61;Dels=0;FS=1.021;HRun=1;HaplotypeScore=0;MQ0=2;MQ=46.92;MQRankSum=1.173;QD=14.1;ReadPosRankSum=-0.008;set=variant2'
```