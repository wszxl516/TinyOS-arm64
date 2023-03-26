#include "feature.h"
#include "arm64.h"
#include "common.h"
#include "config.h"
#include "printf.h"

static cpu_feature features[SMP_CORE_COUNT] ={0};

void parse_featrue(){

	usize data = Main_ID_Register();
	usize cpu_id_data =  CPUID();
	u16 core_id = GET_BITS(cpu_id_data, 0, 7);
	u16 socket_id = GET_BITS(cpu_id_data, 8, 15);
	u16 cluster0 = GET_BITS(cpu_id_data, 16, 23);
	u16 mt = GET_BIT(cpu_id_data, 24);
	u16 cluster1 = GET_BITS(cpu_id_data, 32, 39);
	u32 id = core_id << 0 | socket_id << 8 | cluster0 << 16 | cluster1 << 24;
	features[id].id_num = id;
	features[id].id.core = core_id;
	features[id].id.socket = socket_id;
	features[id].id.cluster[0] = cluster0;
	features[id].id.mt = mt;
	features[id].id.cluster[1] = cluster1;
	features[id].arch = ARCHITECTURE(data);
	features[id].impl = IMPLEMENTER(data);
	features[id].variant = VARIANT(data);
	features[id].partid = PARTNUM(data);
	features[id].revision = MAJOR_REVISION(data);
	features[id].attr[0] = Processor_Feature_Register_0();
	features[id].attr[1] = Processor_Feature_Register_1();

}



void feature_dump(){
    for (u32 i = 0; i < SMP_CORE_COUNT; i++)
	{
		if (features[i].id_num != i)
			continue;
		pr_info("Processor%u \n", features[i].id_num);
		pr_info("\tImpl\t: %s \n", CPU_IMPLEMENTER[features[i].impl]);
		pr_info("\tArch\t: %02x \n", features[i].arch);
		pr_info("\tPart\t: %s \n", CPU_PART[features[i].partid]);
		pr_info("\tVariant\t: %02x \n", features[i].variant);
		pr_info("\tRev\t: %02x\n", features[i].revision);
		pr_info("\tFeatures: ");
		pr_info("%s", FEATURE_CSV3(features[i].attr[0]) ? "CSV3 " : "", features[i].attr[0]);
		pr_info("%s", FEATURE_CSV2(features[i].attr[0]) ? "CSV2 " : "");
		pr_info("%s", FEATURE_DIT(features[i].attr[0])  ? "DIT " : "");
		pr_info("%s", FEATURE_AMU(features[i].attr[0])  ? "AMU " : "");
		pr_info("%s", FEATURE_MPAM(features[i].attr[0]) ? "MPAM " : "");
		pr_info("%s", FEATURE_SEL2(features[i].attr[0]) ? "SEL2 " : "");
		pr_info("%s", FEATURE_SVE(features[i].attr[0])  ? "SVE " : "");
		pr_info("%s", FEATURE_RAS(features[i].attr[0]) ? "RAS " : "");
		pr_info("%s", FEATURE_GIC(features[i].attr[0]) ? "GICV3+ " : "");
		pr_info("%s", FEATURE_SIMD(features[i].attr[0]) >= 1 ? "SIMD " : "");
		pr_info("%s", FEATURE_FP(features[i].attr[0]) != 0b1111 ? "FP " : "");
		pr_info("%s", FEATURE_EL0(features[i].attr[0]) >= 1 ? "EL0 " : "");
		pr_info("%s", FEATURE_EL1(features[i].attr[0]) >= 1 ? "EL1 " : "");
		pr_info("%s", FEATURE_EL2(features[i].attr[0]) >= 1 ? "EL2 " : "");
		pr_info("%s", FEATURE_EL3(features[i].attr[0]) >= 1 ? "EL3 " : "");
		pr_info("%s", FEATURE_PFAR(features[i].attr[1]) ? "PFAR " : "");
		pr_info("%s", FEATURE_DF2(features[i].attr[1]) ? "DF2 " : "");
		pr_info("%s", FEATURE_MTEX(features[i].attr[1]) ? "MTEX " : "");
		pr_info("%s", FEATURE_THE(features[i].attr[1]) ? "THE " : "");
		pr_info("%s", FEATURE_GCS(features[i].attr[1]) ? "GCS " : "");
		pr_info("%s", FEATURE_MTE_FRAC(features[i].attr[1]) ? "MTEFRAC " : "");
		pr_info("%s", FEATURE_NMI(features[i].attr[1]) ? "NMI " : "");
		pr_info("%s", FEATURE_CSV2_FRAC(features[i].attr[1]) ? "CSV2 " : "");
		pr_info("%s", FEATURE_RNDR_TRAP(features[i].attr[1]) ? "RNDRTRAP " : "");
		pr_info("%s", FEATURE_SME(features[i].attr[1]) ? "SME " : "");
		pr_info("%s", FEATURE_MPAM_FRAC(features[i].attr[1]) ? "MPAMFRAC " : "");
		pr_info("%s", FEATURE_RAS_FRAC(features[i].attr[1]) ? "RASFRAC " : "");
		pr_info("%s", FEATURE_MTE(features[i].attr[1]) ? "MTE " : "");
		pr_info("%s", FEATURE_SSBS(features[i].attr[1]) ? "SSbs " : "");
		pr_info("%s", FEATURE_BT(features[i].attr[1]) ? "BT" : "");
		pr_info(".\n\n");

	}
	
}