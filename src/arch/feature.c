#include "feature.h"

#include "arm64.h"
#include "common.h"
#include "config.h"
#include "cputypes.h"
#include "printf.h"
#include "ptable.h"

static cpu_feature features[SMP_CORE_COUNT] = {0};

void parse_featrue() {
  usize data = Main_ID_Register();
  usize cpu_id_data = CPUID();
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

void feature_dump() {
  for (u32 i = 0; i < SMP_CORE_COUNT; i++) {
    if (features[i].id_num != i) continue;
    pr_table("Processor%u", 50, features[i].id_num);
    pr_table("Impl: %s Arch: 0x%x Part: %s", 50,
             CPU_IMPLEMENTER[features[i].impl], features[i].arch,
             CPU_PART[features[i].partid]);
    pr_table("Variant: 0x%x Rev: 0x%x", 50, features[i].variant,
             features[i].revision);
    pr_table("Features", 50);
    char buffer[1024] = {0};
    k_sprintf(buffer,
              "%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s%s",
              FEATURE_CSV3(features[i].attr[0]) ? "CSV3 " : "",
              FEATURE_CSV2(features[i].attr[0]) ? "CSV2 " : "",
              FEATURE_DIT(features[i].attr[0]) ? "DIT " : "",
              FEATURE_AMU(features[i].attr[0]) ? "AMU " : "",
              FEATURE_MPAM(features[i].attr[0]) ? "MPAM " : "",
              FEATURE_SEL2(features[i].attr[0]) ? "SEL2 " : "",
              FEATURE_SVE(features[i].attr[0]) ? "SVE " : "",
              FEATURE_RAS(features[i].attr[0]) ? "RAS " : "",
              FEATURE_GIC(features[i].attr[0]) ? "GICV3+ " : "",
              FEATURE_SIMD(features[i].attr[0]) >= 1 ? "SIMD " : "",
              FEATURE_FP(features[i].attr[0]) != 0b1111 ? "FP " : "",
              FEATURE_EL0(features[i].attr[0]) >= 1 ? "EL0 " : "",
              FEATURE_EL1(features[i].attr[0]) >= 1 ? "EL1 " : "",
              FEATURE_EL2(features[i].attr[0]) >= 1 ? "EL2 " : "",
              FEATURE_EL3(features[i].attr[0]) >= 1 ? "EL3 " : "",
              FEATURE_PFAR(features[i].attr[1]) ? "PFAR " : "",
              FEATURE_DF2(features[i].attr[1]) ? "DF2 " : "",
              FEATURE_MTEX(features[i].attr[1]) ? "MTEX " : "",
              FEATURE_THE(features[i].attr[1]) ? "THE " : "",
              FEATURE_GCS(features[i].attr[1]) ? "GCS " : "",
              FEATURE_MTE_FRAC(features[i].attr[1]) ? "MTEFRAC " : "",
              FEATURE_NMI(features[i].attr[1]) ? "NMI " : "",
              FEATURE_CSV2_FRAC(features[i].attr[1]) ? "CSV2 " : "",
              FEATURE_RNDR_TRAP(features[i].attr[1]) ? "RNDRTRAP " : "",
              FEATURE_SME(features[i].attr[1]) ? "SME " : "",
              FEATURE_MPAM_FRAC(features[i].attr[1]) ? "MPAMFRAC " : "",
              FEATURE_RAS_FRAC(features[i].attr[1]) ? "RASFRAC " : "",
              FEATURE_MTE(features[i].attr[1]) ? "MTE " : "",
              FEATURE_SSBS(features[i].attr[1]) ? "SSbs " : "",
              FEATURE_BT(features[i].attr[1]) ? "BT" : "");
    pr_table("%s", 50, buffer);
    pr_table_end(50);
  }
}