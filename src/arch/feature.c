#include "feature.h"
#include "arm64.h"
#include "common.h"
#include "printf.h"

void processor_feature(){
    usize cpu_id = Main_ID_Register();
    usize feature = Processor_Feature_Register_0();
	pr_info("CPU implementer\t: %s\n", CPU_IMPLEMENTER[IMPLEMENTER(cpu_id)]);
	pr_info("CPU architecture: 8\n");
	pr_info("CPU variant\t: 0x%x\n", VARIANT(cpu_id));
	pr_info("CPU part\t: %s\n", CPU_PART[PARTNUM(cpu_id)]);
	pr_info("CPU revision\t: %d\n", MAJOR_REVISION(cpu_id));
	pr_info("Current El\t: %u\n", CURRENT_EL());
    pr_info("Features: ");
	pr_info("%s ", FEATURE_GIC(feature) ? "GICV3+" : "NoGIC");
	pr_info("%s ", FEATURE_RAS(feature) ? "RAS" : "");
	pr_info("%s ", FEATURE_FP(feature) != 0b1111 ? "FP" : "");
	pr_info("%s ", FEATURE_SIMD(feature) >= 1 ? "SIMD" : "");
	pr_info("%s ", FEATURE_EL0(feature) >= 1 ? "EL0" : "");
	pr_info("%s ", FEATURE_EL1(feature) >= 1 ? "EL1" : "");
	pr_info("%s ", FEATURE_EL2(feature) >= 1 ? "EL2" : "");
	pr_info("%s ", FEATURE_EL3(feature) >= 1 ? "EL3" : "");
	pr_info("...\n");
}