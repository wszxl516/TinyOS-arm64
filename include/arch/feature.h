
#ifndef __FEATURE_H__
#define __FEATURE_H__
#include "cputypes.h"
//Provides identification information for the PE, including an implementer code for the device and a device ID number.
#define Main_ID_Register()      REG_READ_P(MIDR_EL1)
#define MAJOR_REVISION(data)    GET_BITS(data, 0, 3)
#define PARTNUM(data)           GET_BITS(data, 4, 15)
#define ARCHITECTURE(data)      GET_BITS(data, 16, 19)
#define VARIANT(data)           GET_BITS(data, 20, 23)
#define IMPLEMENTER(data)       GET_BITS(data, 24, 31)

//Provides implementation-specific minor revision information.
#define Revision_ID_Register()          REG_READ_P(REVIDR_EL1)


//AArch64 Processor Feature Register
#define Processor_Feature_Register_0()      REG_READ_P(ID_AA64PFR0_EL1)
#define FEATURE_CSV3(data)                  GET_BITS(data, 60, 63)
#define FEATURE_CSV2(data)                  GET_BITS(data, 56, 59)
#define FEATURE_RME(data)                   GET_BITS(data, 52, 55)
#define FEATURE_DIT(data)                   GET_BITS(data, 48, 51)
#define FEATURE_AMU(data)                   GET_BITS(data, 44, 47)
#define FEATURE_MPAM(data)                  GET_BITS(data, 40, 43)
#define FEATURE_SEL2(data)                  GET_BITS(data, 36, 39)
#define FEATURE_SVE(data)                   GET_BITS(data, 32, 35)
#define FEATURE_RAS(data)                   GET_BITS(data, 28, 31)
#define FEATURE_GIC(data)                   GET_BITS(data, 24, 27)
#define FEATURE_SIMD(data)                  GET_BITS(data, 20, 23)
#define FEATURE_FP(data)                    GET_BITS(data, 16, 19)
#define FEATURE_EL3(data)                   GET_BITS(data, 12, 15)
#define FEATURE_EL2(data)                   GET_BITS(data, 8, 11)
#define FEATURE_EL1(data)                   GET_BITS(data, 4, 7)
#define FEATURE_EL0(data)                   GET_BITS(data, 0, 3)



#define Processor_Feature_Register_1()  REG_READ_P(ID_AA64PFR1_EL1)
#define FEATURE_PFAR(data)                  GET_BITS(data, 60, 63)
#define FEATURE_DF2(data)                   GET_BITS(data, 56, 59)
#define FEATURE_MTEX(data)                  GET_BITS(data, 52, 55)
#define FEATURE_THE(data)                   GET_BITS(data, 48, 51)
#define FEATURE_GCS(data)                   GET_BITS(data, 44, 47)
#define FEATURE_MTE_FRAC(data)              GET_BITS(data, 40, 43)
#define FEATURE_NMI(data)                   GET_BITS(data, 36, 39)
#define FEATURE_CSV2_FRAC(data)             GET_BITS(data, 32, 35)
#define FEATURE_RNDR_TRAP(data)             GET_BITS(data, 28, 31)
#define FEATURE_SME(data)                   GET_BITS(data, 24, 27)
#define FEATURE_RESERVED
#define FEATURE_MPAM_FRAC(data)             GET_BITS(data, 16, 19)
#define FEATURE_RAS_FRAC(data)              GET_BITS(data, 12, 15)
#define FEATURE_MTE(data)                   GET_BITS(data, 8, 11)
#define FEATURE_SSBS(data)                  GET_BITS(data, 4, 7)
#define FEATURE_BT(data)                    GET_BITS(data, 0, 3)


#define Processor_Feature_Register_2()       REG_READ_P(ID_AA64PFR2_EL1)
#define FEATURE_MTEFAR(data)                 GET_BITS(data, 8, 11)
#define FEATURE_MTESTOREONLY(data)           GET_BITS(data, 4, 7)
#define FEATURE_MTEPERM(data)                GET_BITS(data, 0, 3)

void processor_feature();

#endif //__FEATURE_H__