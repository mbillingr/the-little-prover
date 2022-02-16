#![allow(non_snake_case)]
#![allow(unused_variables)]

use crate::jbob_runtime::*;

const C_QUOTE: S<'static> = S::Symbol("quote");
const C_IF: S<'static> = S::Symbol("if");
const C_DEFUN: S<'static> = S::Symbol("defun");
const C_DETHM: S<'static> = S::Symbol("dethm");
const C__LT: S<'static> = S::Symbol("<");
const C__PLUS: S<'static> = S::Symbol("+");
const C_SIZE: S<'static> = S::Symbol("size");
const C_NATP: S<'static> = S::Symbol("natp");
const C_CONS: S<'static> = S::Symbol("cons");
const C_CDR: S<'static> = S::Symbol("cdr");
const C_CAR: S<'static> = S::Symbol("car");
const C_ATOM: S<'static> = S::Symbol("atom");
const C_EQUAL: S<'static> = S::Symbol("equal");
const C_X: S<'static> = S::Symbol("x");
const C_Y: S<'static> = S::Symbol("y");
const C_ANY: S<'static> = S::Symbol("any");
const C_E: S<'static> = S::Symbol("E");
const C_A: S<'static> = S::Symbol("A");
const C_Q: S<'static> = S::Symbol("Q");
const C_IDENTITY__PLUS: S<'static> = S::Symbol("identity-+");
const C_Z: S<'static> = S::Symbol("z");
const C_COMMON_ADDENDS__LT: S<'static> = S::Symbol("common-addends-<");
const C_POSITIVES__PLUS: S<'static> = S::Symbol("positives-+");
const C_NATP_SLASH__PLUS: S<'static> = S::Symbol("natp/+");
const C_COMMUTE__PLUS: S<'static> = S::Symbol("commute-+");
const C_C: S<'static> = S::Symbol("c");
const C_B: S<'static> = S::Symbol("b");
const C_ASSOCIATE__PLUS: S<'static> = S::Symbol("associate-+");
const C_SIZE_SLASH_CDR: S<'static> = S::Symbol("size/cdr");
const C_SIZE_SLASH_CAR: S<'static> = S::Symbol("size/car");
const C_NATP_SLASH_SIZE: S<'static> = S::Symbol("natp/size");
const C_EQUAL_IF: S<'static> = S::Symbol("equal-if");
const C_CONS_SLASH_CAR_PLUS_CDR: S<'static> = S::Symbol("cons/car+cdr");
const C_IF_NEST_A: S<'static> = S::Symbol("if-nest-A");
const C_IF_NEST_E: S<'static> = S::Symbol("if-nest-E");
const C_IF_FALSE: S<'static> = S::Symbol("if-false");
const C_IF_TRUE: S<'static> = S::Symbol("if-true");
const C_IF_SAME: S<'static> = S::Symbol("if-same");
const C_EQUAL_SWAP: S<'static> = S::Symbol("equal-swap");
const C_EQUAL_SAME: S<'static> = S::Symbol("equal-same");
const C_CDR_SLASH_CONS: S<'static> = S::Symbol("cdr/cons");
const C_CAR_SLASH_CONS: S<'static> = S::Symbol("car/cons");
const C_ATOM_SLASH_CONS: S<'static> = S::Symbol("atom/cons");
const C_STAR_INDUCTION: S<'static> = S::Symbol("star-induction");
const C_LIST_INDUCTION: S<'static> = S::Symbol("list-induction");
const C_PAIR_0: S<'static> = S::Pair(&(C__LT, S::Empty));
const C_PAIR_1: S<'static> = S::Pair(&(C__PLUS, C_PAIR_0));
const C_PAIR_2: S<'static> = S::Pair(&(C_SIZE, C_PAIR_1));
const C_PAIR_3: S<'static> = S::Pair(&(C_NATP, C_PAIR_2));
const C_PAIR_4: S<'static> = S::Pair(&(C_CONS, C_PAIR_3));
const C_PAIR_5: S<'static> = S::Pair(&(C_CDR, C_PAIR_4));
const C_PAIR_6: S<'static> = S::Pair(&(C_CAR, C_PAIR_5));
const C_PAIR_7: S<'static> = S::Pair(&(C_ATOM, C_PAIR_6));
const C_PAIR_8: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_7));
const C_PAIR_9: S<'static> = S::Pair(&(C_SIZE, S::Empty));
const C_PAIR_10: S<'static> = S::Pair(&(C_NATP, C_PAIR_9));
const C_PAIR_11: S<'static> = S::Pair(&(C_CDR, C_PAIR_10));
const C_PAIR_12: S<'static> = S::Pair(&(C_CAR, C_PAIR_11));
const C_PAIR_13: S<'static> = S::Pair(&(C_ATOM, C_PAIR_12));
const C_PAIR_14: S<'static> = S::Pair(&(C_X, S::Empty));
const C_PAIR_15: S<'static> = S::Pair(&(C_CONS, C_PAIR_1));
const C_PAIR_16: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_15));
const C_PAIR_17: S<'static> = S::Pair(&(C_Y, S::Empty));
const C_PAIR_18: S<'static> = S::Pair(&(C_X, C_PAIR_17));
const C_PAIR_19: S<'static> = S::Pair(&(C_E, S::Empty));
const C_PAIR_20: S<'static> = S::Pair(&(C_A, C_PAIR_19));
const C_PAIR_21: S<'static> = S::Pair(&(C_Q, C_PAIR_20));
const C_PAIR_22: S<'static> = S::Pair(&(C_T, S::Empty));
const C_PAIR_23: S<'static> = S::Pair(&(C_QUOTE, C_PAIR_22));
const C_PAIR_24: S<'static> = S::Pair(&(C_PAIR_23, S::Empty));
const C_PAIR_25: S<'static> = S::Pair(&(S::Num(0), S::Empty));
const C_PAIR_26: S<'static> = S::Pair(&(C_QUOTE, C_PAIR_25));
const C_PAIR_27: S<'static> = S::Pair(&(C_PAIR_26, C_PAIR_14));
const C_PAIR_28: S<'static> = S::Pair(&(C__PLUS, C_PAIR_27));
const C_PAIR_29: S<'static> = S::Pair(&(C_PAIR_28, C_PAIR_14));
const C_PAIR_30: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_29));
const C_PAIR_31: S<'static> = S::Pair(&(C_PAIR_30, C_PAIR_24));
const C_PAIR_32: S<'static> = S::Pair(&(C_NATP, C_PAIR_14));
const C_PAIR_33: S<'static> = S::Pair(&(C_PAIR_32, C_PAIR_31));
const C_PAIR_34: S<'static> = S::Pair(&(C_IF, C_PAIR_33));
const C_PAIR_35: S<'static> = S::Pair(&(C_PAIR_34, S::Empty));
const C_PAIR_36: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_35));
const C_PAIR_37: S<'static> = S::Pair(&(C_IDENTITY__PLUS, C_PAIR_36));
const C_PAIR_38: S<'static> = S::Pair(&(C_DETHM, C_PAIR_37));
const C_PAIR_39: S<'static> = S::Pair(&(C_PAIR_38, S::Empty));
const C_PAIR_40: S<'static> = S::Pair(&(C__LT, C_PAIR_18));
const C_PAIR_41: S<'static> = S::Pair(&(C_PAIR_40, S::Empty));
const C_PAIR_42: S<'static> = S::Pair(&(C_Z, S::Empty));
const C_PAIR_43: S<'static> = S::Pair(&(C_Y, C_PAIR_42));
const C_PAIR_44: S<'static> = S::Pair(&(C__PLUS, C_PAIR_43));
const C_PAIR_45: S<'static> = S::Pair(&(C_PAIR_44, S::Empty));
const C_PAIR_46: S<'static> = S::Pair(&(C_X, C_PAIR_42));
const C_PAIR_47: S<'static> = S::Pair(&(C__PLUS, C_PAIR_46));
const C_PAIR_48: S<'static> = S::Pair(&(C_PAIR_47, C_PAIR_45));
const C_PAIR_49: S<'static> = S::Pair(&(C__LT, C_PAIR_48));
const C_PAIR_50: S<'static> = S::Pair(&(C_PAIR_49, C_PAIR_41));
const C_PAIR_51: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_50));
const C_PAIR_52: S<'static> = S::Pair(&(C_PAIR_51, S::Empty));
const C_PAIR_53: S<'static> = S::Pair(&(C_X, C_PAIR_43));
const C_PAIR_54: S<'static> = S::Pair(&(C_PAIR_53, C_PAIR_52));
const C_PAIR_55: S<'static> = S::Pair(&(C_COMMON_ADDENDS__LT, C_PAIR_54));
const C_PAIR_56: S<'static> = S::Pair(&(C_DETHM, C_PAIR_55));
const C_PAIR_57: S<'static> = S::Pair(&(C_PAIR_56, C_PAIR_39));
const C_PAIR_58: S<'static> = S::Pair(&(C__PLUS, C_PAIR_18));
const C_PAIR_59: S<'static> = S::Pair(&(C_PAIR_58, S::Empty));
const C_PAIR_60: S<'static> = S::Pair(&(C_PAIR_26, C_PAIR_59));
const C_PAIR_61: S<'static> = S::Pair(&(C__LT, C_PAIR_60));
const C_PAIR_62: S<'static> = S::Pair(&(C_PAIR_61, C_PAIR_24));
const C_PAIR_63: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_62));
const C_PAIR_64: S<'static> = S::Pair(&(C_PAIR_63, C_PAIR_24));
const C_PAIR_65: S<'static> = S::Pair(&(C_PAIR_26, C_PAIR_17));
const C_PAIR_66: S<'static> = S::Pair(&(C__LT, C_PAIR_65));
const C_PAIR_67: S<'static> = S::Pair(&(C_PAIR_66, C_PAIR_64));
const C_PAIR_68: S<'static> = S::Pair(&(C_IF, C_PAIR_67));
const C_PAIR_69: S<'static> = S::Pair(&(C_PAIR_68, C_PAIR_24));
const C_PAIR_70: S<'static> = S::Pair(&(C__LT, C_PAIR_27));
const C_PAIR_71: S<'static> = S::Pair(&(C_PAIR_70, C_PAIR_69));
const C_PAIR_72: S<'static> = S::Pair(&(C_IF, C_PAIR_71));
const C_PAIR_73: S<'static> = S::Pair(&(C_PAIR_72, S::Empty));
const C_PAIR_74: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_73));
const C_PAIR_75: S<'static> = S::Pair(&(C_POSITIVES__PLUS, C_PAIR_74));
const C_PAIR_76: S<'static> = S::Pair(&(C_DETHM, C_PAIR_75));
const C_PAIR_77: S<'static> = S::Pair(&(C_PAIR_76, C_PAIR_57));
const C_PAIR_78: S<'static> = S::Pair(&(C_NATP, C_PAIR_59));
const C_PAIR_79: S<'static> = S::Pair(&(C_PAIR_78, C_PAIR_24));
const C_PAIR_80: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_79));
const C_PAIR_81: S<'static> = S::Pair(&(C_PAIR_80, C_PAIR_24));
const C_PAIR_82: S<'static> = S::Pair(&(C_NATP, C_PAIR_17));
const C_PAIR_83: S<'static> = S::Pair(&(C_PAIR_82, C_PAIR_81));
const C_PAIR_84: S<'static> = S::Pair(&(C_IF, C_PAIR_83));
const C_PAIR_85: S<'static> = S::Pair(&(C_PAIR_84, C_PAIR_24));
const C_PAIR_86: S<'static> = S::Pair(&(C_PAIR_32, C_PAIR_85));
const C_PAIR_87: S<'static> = S::Pair(&(C_IF, C_PAIR_86));
const C_PAIR_88: S<'static> = S::Pair(&(C_PAIR_87, S::Empty));
const C_PAIR_89: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_88));
const C_PAIR_90: S<'static> = S::Pair(&(C_NATP_SLASH__PLUS, C_PAIR_89));
const C_PAIR_91: S<'static> = S::Pair(&(C_DETHM, C_PAIR_90));
const C_PAIR_92: S<'static> = S::Pair(&(C_PAIR_91, C_PAIR_77));
const C_PAIR_93: S<'static> = S::Pair(&(C_Y, C_PAIR_14));
const C_PAIR_94: S<'static> = S::Pair(&(C__PLUS, C_PAIR_93));
const C_PAIR_95: S<'static> = S::Pair(&(C_PAIR_94, S::Empty));
const C_PAIR_96: S<'static> = S::Pair(&(C_PAIR_58, C_PAIR_95));
const C_PAIR_97: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_96));
const C_PAIR_98: S<'static> = S::Pair(&(C_PAIR_97, S::Empty));
const C_PAIR_99: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_98));
const C_PAIR_100: S<'static> = S::Pair(&(C_COMMUTE__PLUS, C_PAIR_99));
const C_PAIR_101: S<'static> = S::Pair(&(C_DETHM, C_PAIR_100));
const C_PAIR_102: S<'static> = S::Pair(&(C_PAIR_101, C_PAIR_92));
const C_PAIR_103: S<'static> = S::Pair(&(C_C, S::Empty));
const C_PAIR_104: S<'static> = S::Pair(&(C_B, C_PAIR_103));
const C_PAIR_105: S<'static> = S::Pair(&(C__PLUS, C_PAIR_104));
const C_PAIR_106: S<'static> = S::Pair(&(C_PAIR_105, S::Empty));
const C_PAIR_107: S<'static> = S::Pair(&(C_A, C_PAIR_106));
const C_PAIR_108: S<'static> = S::Pair(&(C__PLUS, C_PAIR_107));
const C_PAIR_109: S<'static> = S::Pair(&(C_PAIR_108, S::Empty));
const C_PAIR_110: S<'static> = S::Pair(&(C_B, S::Empty));
const C_PAIR_111: S<'static> = S::Pair(&(C_A, C_PAIR_110));
const C_PAIR_112: S<'static> = S::Pair(&(C__PLUS, C_PAIR_111));
const C_PAIR_113: S<'static> = S::Pair(&(C_PAIR_112, C_PAIR_103));
const C_PAIR_114: S<'static> = S::Pair(&(C__PLUS, C_PAIR_113));
const C_PAIR_115: S<'static> = S::Pair(&(C_PAIR_114, C_PAIR_109));
const C_PAIR_116: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_115));
const C_PAIR_117: S<'static> = S::Pair(&(C_PAIR_116, S::Empty));
const C_PAIR_118: S<'static> = S::Pair(&(C_A, C_PAIR_104));
const C_PAIR_119: S<'static> = S::Pair(&(C_PAIR_118, C_PAIR_117));
const C_PAIR_120: S<'static> = S::Pair(&(C_ASSOCIATE__PLUS, C_PAIR_119));
const C_PAIR_121: S<'static> = S::Pair(&(C_DETHM, C_PAIR_120));
const C_PAIR_122: S<'static> = S::Pair(&(C_PAIR_121, C_PAIR_102));
const C_PAIR_123: S<'static> = S::Pair(&(C_SIZE, C_PAIR_14));
const C_PAIR_124: S<'static> = S::Pair(&(C_PAIR_123, S::Empty));
const C_PAIR_125: S<'static> = S::Pair(&(C_CDR, C_PAIR_14));
const C_PAIR_126: S<'static> = S::Pair(&(C_PAIR_125, S::Empty));
const C_PAIR_127: S<'static> = S::Pair(&(C_SIZE, C_PAIR_126));
const C_PAIR_128: S<'static> = S::Pair(&(C_PAIR_127, C_PAIR_124));
const C_PAIR_129: S<'static> = S::Pair(&(C__LT, C_PAIR_128));
const C_PAIR_130: S<'static> = S::Pair(&(C_PAIR_129, C_PAIR_24));
const C_PAIR_131: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_130));
const C_PAIR_132: S<'static> = S::Pair(&(C_PAIR_131, S::Empty));
const C_PAIR_133: S<'static> = S::Pair(&(C_PAIR_23, C_PAIR_132));
const C_PAIR_134: S<'static> = S::Pair(&(C_ATOM, C_PAIR_14));
const C_PAIR_135: S<'static> = S::Pair(&(C_PAIR_134, C_PAIR_133));
const C_PAIR_136: S<'static> = S::Pair(&(C_IF, C_PAIR_135));
const C_PAIR_137: S<'static> = S::Pair(&(C_PAIR_136, S::Empty));
const C_PAIR_138: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_137));
const C_PAIR_139: S<'static> = S::Pair(&(C_SIZE_SLASH_CDR, C_PAIR_138));
const C_PAIR_140: S<'static> = S::Pair(&(C_DETHM, C_PAIR_139));
const C_PAIR_141: S<'static> = S::Pair(&(C_PAIR_140, C_PAIR_122));
const C_PAIR_142: S<'static> = S::Pair(&(C_CAR, C_PAIR_14));
const C_PAIR_143: S<'static> = S::Pair(&(C_PAIR_142, S::Empty));
const C_PAIR_144: S<'static> = S::Pair(&(C_SIZE, C_PAIR_143));
const C_PAIR_145: S<'static> = S::Pair(&(C_PAIR_144, C_PAIR_124));
const C_PAIR_146: S<'static> = S::Pair(&(C__LT, C_PAIR_145));
const C_PAIR_147: S<'static> = S::Pair(&(C_PAIR_146, C_PAIR_24));
const C_PAIR_148: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_147));
const C_PAIR_149: S<'static> = S::Pair(&(C_PAIR_148, S::Empty));
const C_PAIR_150: S<'static> = S::Pair(&(C_PAIR_23, C_PAIR_149));
const C_PAIR_151: S<'static> = S::Pair(&(C_PAIR_134, C_PAIR_150));
const C_PAIR_152: S<'static> = S::Pair(&(C_IF, C_PAIR_151));
const C_PAIR_153: S<'static> = S::Pair(&(C_PAIR_152, S::Empty));
const C_PAIR_154: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_153));
const C_PAIR_155: S<'static> = S::Pair(&(C_SIZE_SLASH_CAR, C_PAIR_154));
const C_PAIR_156: S<'static> = S::Pair(&(C_DETHM, C_PAIR_155));
const C_PAIR_157: S<'static> = S::Pair(&(C_PAIR_156, C_PAIR_141));
const C_PAIR_158: S<'static> = S::Pair(&(C_NATP, C_PAIR_124));
const C_PAIR_159: S<'static> = S::Pair(&(C_PAIR_158, C_PAIR_24));
const C_PAIR_160: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_159));
const C_PAIR_161: S<'static> = S::Pair(&(C_PAIR_160, S::Empty));
const C_PAIR_162: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_161));
const C_PAIR_163: S<'static> = S::Pair(&(C_NATP_SLASH_SIZE, C_PAIR_162));
const C_PAIR_164: S<'static> = S::Pair(&(C_DETHM, C_PAIR_163));
const C_PAIR_165: S<'static> = S::Pair(&(C_PAIR_164, C_PAIR_157));
const C_PAIR_166: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_18));
const C_PAIR_167: S<'static> = S::Pair(&(C_PAIR_166, C_PAIR_24));
const C_PAIR_168: S<'static> = S::Pair(&(C_PAIR_166, C_PAIR_167));
const C_PAIR_169: S<'static> = S::Pair(&(C_IF, C_PAIR_168));
const C_PAIR_170: S<'static> = S::Pair(&(C_PAIR_169, S::Empty));
const C_PAIR_171: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_170));
const C_PAIR_172: S<'static> = S::Pair(&(C_EQUAL_IF, C_PAIR_171));
const C_PAIR_173: S<'static> = S::Pair(&(C_DETHM, C_PAIR_172));
const C_PAIR_174: S<'static> = S::Pair(&(C_PAIR_173, C_PAIR_165));
const C_PAIR_175: S<'static> = S::Pair(&(C_PAIR_142, C_PAIR_126));
const C_PAIR_176: S<'static> = S::Pair(&(C_CONS, C_PAIR_175));
const C_PAIR_177: S<'static> = S::Pair(&(C_PAIR_176, C_PAIR_14));
const C_PAIR_178: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_177));
const C_PAIR_179: S<'static> = S::Pair(&(C_PAIR_178, S::Empty));
const C_PAIR_180: S<'static> = S::Pair(&(C_PAIR_23, C_PAIR_179));
const C_PAIR_181: S<'static> = S::Pair(&(C_PAIR_134, C_PAIR_180));
const C_PAIR_182: S<'static> = S::Pair(&(C_IF, C_PAIR_181));
const C_PAIR_183: S<'static> = S::Pair(&(C_PAIR_182, S::Empty));
const C_PAIR_184: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_183));
const C_PAIR_185: S<'static> = S::Pair(&(C_CONS_SLASH_CAR_PLUS_CDR, C_PAIR_184));
const C_PAIR_186: S<'static> = S::Pair(&(C_DETHM, C_PAIR_185));
const C_PAIR_187: S<'static> = S::Pair(&(C_PAIR_186, C_PAIR_174));
const C_PAIR_188: S<'static> = S::Pair(&(C_IF, C_PAIR_53));
const C_PAIR_189: S<'static> = S::Pair(&(C_PAIR_188, C_PAIR_17));
const C_PAIR_190: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_189));
const C_PAIR_191: S<'static> = S::Pair(&(C_PAIR_190, C_PAIR_24));
const C_PAIR_192: S<'static> = S::Pair(&(C_X, C_PAIR_191));
const C_PAIR_193: S<'static> = S::Pair(&(C_IF, C_PAIR_192));
const C_PAIR_194: S<'static> = S::Pair(&(C_PAIR_193, S::Empty));
const C_PAIR_195: S<'static> = S::Pair(&(C_PAIR_53, C_PAIR_194));
const C_PAIR_196: S<'static> = S::Pair(&(C_IF_NEST_A, C_PAIR_195));
const C_PAIR_197: S<'static> = S::Pair(&(C_DETHM, C_PAIR_196));
const C_PAIR_198: S<'static> = S::Pair(&(C_PAIR_197, C_PAIR_187));
const C_PAIR_199: S<'static> = S::Pair(&(C_PAIR_188, C_PAIR_42));
const C_PAIR_200: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_199));
const C_PAIR_201: S<'static> = S::Pair(&(C_PAIR_200, S::Empty));
const C_PAIR_202: S<'static> = S::Pair(&(C_PAIR_23, C_PAIR_201));
const C_PAIR_203: S<'static> = S::Pair(&(C_X, C_PAIR_202));
const C_PAIR_204: S<'static> = S::Pair(&(C_IF, C_PAIR_203));
const C_PAIR_205: S<'static> = S::Pair(&(C_PAIR_204, S::Empty));
const C_PAIR_206: S<'static> = S::Pair(&(C_PAIR_53, C_PAIR_205));
const C_PAIR_207: S<'static> = S::Pair(&(C_IF_NEST_E, C_PAIR_206));
const C_PAIR_208: S<'static> = S::Pair(&(C_DETHM, C_PAIR_207));
const C_PAIR_209: S<'static> = S::Pair(&(C_PAIR_208, C_PAIR_198));
const C_PAIR_210: S<'static> = S::Pair(&(C_NIL, S::Empty));
const C_PAIR_211: S<'static> = S::Pair(&(C_QUOTE, C_PAIR_210));
const C_PAIR_212: S<'static> = S::Pair(&(C_PAIR_211, C_PAIR_18));
const C_PAIR_213: S<'static> = S::Pair(&(C_IF, C_PAIR_212));
const C_PAIR_214: S<'static> = S::Pair(&(C_PAIR_213, C_PAIR_17));
const C_PAIR_215: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_214));
const C_PAIR_216: S<'static> = S::Pair(&(C_PAIR_215, S::Empty));
const C_PAIR_217: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_216));
const C_PAIR_218: S<'static> = S::Pair(&(C_IF_FALSE, C_PAIR_217));
const C_PAIR_219: S<'static> = S::Pair(&(C_DETHM, C_PAIR_218));
const C_PAIR_220: S<'static> = S::Pair(&(C_PAIR_219, C_PAIR_209));
const C_PAIR_221: S<'static> = S::Pair(&(C_PAIR_23, C_PAIR_18));
const C_PAIR_222: S<'static> = S::Pair(&(C_IF, C_PAIR_221));
const C_PAIR_223: S<'static> = S::Pair(&(C_PAIR_222, C_PAIR_14));
const C_PAIR_224: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_223));
const C_PAIR_225: S<'static> = S::Pair(&(C_PAIR_224, S::Empty));
const C_PAIR_226: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_225));
const C_PAIR_227: S<'static> = S::Pair(&(C_IF_TRUE, C_PAIR_226));
const C_PAIR_228: S<'static> = S::Pair(&(C_DETHM, C_PAIR_227));
const C_PAIR_229: S<'static> = S::Pair(&(C_PAIR_228, C_PAIR_220));
const C_PAIR_230: S<'static> = S::Pair(&(C_Y, C_PAIR_17));
const C_PAIR_231: S<'static> = S::Pair(&(C_X, C_PAIR_230));
const C_PAIR_232: S<'static> = S::Pair(&(C_IF, C_PAIR_231));
const C_PAIR_233: S<'static> = S::Pair(&(C_PAIR_232, C_PAIR_17));
const C_PAIR_234: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_233));
const C_PAIR_235: S<'static> = S::Pair(&(C_PAIR_234, S::Empty));
const C_PAIR_236: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_235));
const C_PAIR_237: S<'static> = S::Pair(&(C_IF_SAME, C_PAIR_236));
const C_PAIR_238: S<'static> = S::Pair(&(C_DETHM, C_PAIR_237));
const C_PAIR_239: S<'static> = S::Pair(&(C_PAIR_238, C_PAIR_229));
const C_PAIR_240: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_93));
const C_PAIR_241: S<'static> = S::Pair(&(C_PAIR_240, S::Empty));
const C_PAIR_242: S<'static> = S::Pair(&(C_PAIR_166, C_PAIR_241));
const C_PAIR_243: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_242));
const C_PAIR_244: S<'static> = S::Pair(&(C_PAIR_243, S::Empty));
const C_PAIR_245: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_244));
const C_PAIR_246: S<'static> = S::Pair(&(C_EQUAL_SWAP, C_PAIR_245));
const C_PAIR_247: S<'static> = S::Pair(&(C_DETHM, C_PAIR_246));
const C_PAIR_248: S<'static> = S::Pair(&(C_PAIR_247, C_PAIR_239));
const C_PAIR_249: S<'static> = S::Pair(&(C_X, C_PAIR_14));
const C_PAIR_250: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_249));
const C_PAIR_251: S<'static> = S::Pair(&(C_PAIR_250, C_PAIR_24));
const C_PAIR_252: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_251));
const C_PAIR_253: S<'static> = S::Pair(&(C_PAIR_252, S::Empty));
const C_PAIR_254: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_253));
const C_PAIR_255: S<'static> = S::Pair(&(C_EQUAL_SAME, C_PAIR_254));
const C_PAIR_256: S<'static> = S::Pair(&(C_DETHM, C_PAIR_255));
const C_PAIR_257: S<'static> = S::Pair(&(C_PAIR_256, C_PAIR_248));
const C_PAIR_258: S<'static> = S::Pair(&(C_CONS, C_PAIR_18));
const C_PAIR_259: S<'static> = S::Pair(&(C_PAIR_258, S::Empty));
const C_PAIR_260: S<'static> = S::Pair(&(C_CDR, C_PAIR_259));
const C_PAIR_261: S<'static> = S::Pair(&(C_PAIR_260, C_PAIR_17));
const C_PAIR_262: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_261));
const C_PAIR_263: S<'static> = S::Pair(&(C_PAIR_262, S::Empty));
const C_PAIR_264: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_263));
const C_PAIR_265: S<'static> = S::Pair(&(C_CDR_SLASH_CONS, C_PAIR_264));
const C_PAIR_266: S<'static> = S::Pair(&(C_DETHM, C_PAIR_265));
const C_PAIR_267: S<'static> = S::Pair(&(C_PAIR_266, C_PAIR_257));
const C_PAIR_268: S<'static> = S::Pair(&(C_CAR, C_PAIR_259));
const C_PAIR_269: S<'static> = S::Pair(&(C_PAIR_268, C_PAIR_14));
const C_PAIR_270: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_269));
const C_PAIR_271: S<'static> = S::Pair(&(C_PAIR_270, S::Empty));
const C_PAIR_272: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_271));
const C_PAIR_273: S<'static> = S::Pair(&(C_CAR_SLASH_CONS, C_PAIR_272));
const C_PAIR_274: S<'static> = S::Pair(&(C_DETHM, C_PAIR_273));
const C_PAIR_275: S<'static> = S::Pair(&(C_PAIR_274, C_PAIR_267));
const C_PAIR_276: S<'static> = S::Pair(&(C_PAIR_211, S::Empty));
const C_PAIR_277: S<'static> = S::Pair(&(C_ATOM, C_PAIR_259));
const C_PAIR_278: S<'static> = S::Pair(&(C_PAIR_277, C_PAIR_276));
const C_PAIR_279: S<'static> = S::Pair(&(C_EQUAL, C_PAIR_278));
const C_PAIR_280: S<'static> = S::Pair(&(C_PAIR_279, S::Empty));
const C_PAIR_281: S<'static> = S::Pair(&(C_PAIR_18, C_PAIR_280));
const C_PAIR_282: S<'static> = S::Pair(&(C_ATOM_SLASH_CONS, C_PAIR_281));
const C_PAIR_283: S<'static> = S::Pair(&(C_DETHM, C_PAIR_282));
const C_PAIR_284: S<'static> = S::Pair(&(C_PAIR_283, C_PAIR_275));
const C_PAIR_285: S<'static> = S::Pair(&(C_PAIR_23, C_PAIR_276));
const C_PAIR_286: S<'static> = S::Pair(&(C_IF_TRUE, C_PAIR_285));
const C_PAIR_287: S<'static> = S::Pair(&(C_PAIR_286, S::Empty));
const C_PAIR_288: S<'static> = S::Pair(&(S::Empty, C_PAIR_287));
const C_PAIR_289: S<'static> = S::Pair(&(C_PAIR_288, S::Empty));
const C_PAIR_290: S<'static> = S::Pair(&(C_NATP_SLASH_SIZE, C_PAIR_14));
const C_PAIR_291: S<'static> = S::Pair(&(C_PAIR_290, S::Empty));
const C_PAIR_292: S<'static> = S::Pair(&(C_Q, S::Empty));
const C_PAIR_293: S<'static> = S::Pair(&(C_PAIR_292, C_PAIR_291));
const C_PAIR_294: S<'static> = S::Pair(&(C_PAIR_293, C_PAIR_289));
const C_PAIR_295: S<'static> = S::Pair(&(C_PAIR_134, C_PAIR_24));
const C_PAIR_296: S<'static> = S::Pair(&(C_IF_SAME, C_PAIR_295));
const C_PAIR_297: S<'static> = S::Pair(&(C_PAIR_296, S::Empty));
const C_PAIR_298: S<'static> = S::Pair(&(C_A, S::Empty));
const C_PAIR_299: S<'static> = S::Pair(&(C_PAIR_298, C_PAIR_297));
const C_PAIR_300: S<'static> = S::Pair(&(C_PAIR_299, C_PAIR_294));
const C_PAIR_301: S<'static> = S::Pair(&(C_PAIR_20, C_PAIR_287));
const C_PAIR_302: S<'static> = S::Pair(&(C_PAIR_301, C_PAIR_300));
const C_PAIR_303: S<'static> = S::Pair(&(C_SIZE_SLASH_CAR, C_PAIR_14));
const C_PAIR_304: S<'static> = S::Pair(&(C_PAIR_303, S::Empty));
const C_PAIR_305: S<'static> = S::Pair(&(C_E, C_PAIR_292));
const C_PAIR_306: S<'static> = S::Pair(&(C_A, C_PAIR_305));
const C_PAIR_307: S<'static> = S::Pair(&(C_PAIR_306, C_PAIR_304));
const C_PAIR_308: S<'static> = S::Pair(&(C_PAIR_307, C_PAIR_302));
const C_PAIR_309: S<'static> = S::Pair(&(C_SIZE_SLASH_CDR, C_PAIR_14));
const C_PAIR_310: S<'static> = S::Pair(&(C_PAIR_309, S::Empty));
const C_PAIR_311: S<'static> = S::Pair(&(C_E, C_PAIR_298));
const C_PAIR_312: S<'static> = S::Pair(&(C_A, C_PAIR_311));
const C_PAIR_313: S<'static> = S::Pair(&(C_PAIR_312, C_PAIR_310));
const C_PAIR_314: S<'static> = S::Pair(&(C_PAIR_313, C_PAIR_308));
const C_PAIR_315: S<'static> = S::Pair(&(C_PAIR_123, C_PAIR_314));
const C_PAIR_316: S<'static> = S::Pair(&(C_STAR_INDUCTION, C_PAIR_126));
const C_PAIR_317: S<'static> = S::Pair(&(C_PAIR_316, S::Empty));
const C_PAIR_318: S<'static> = S::Pair(&(C_STAR_INDUCTION, C_PAIR_143));
const C_PAIR_319: S<'static> = S::Pair(&(C_PAIR_318, C_PAIR_317));
const C_PAIR_320: S<'static> = S::Pair(&(C_CONS, C_PAIR_319));
const C_PAIR_321: S<'static> = S::Pair(&(C_PAIR_320, S::Empty));
const C_PAIR_322: S<'static> = S::Pair(&(C_X, C_PAIR_321));
const C_PAIR_323: S<'static> = S::Pair(&(C_PAIR_134, C_PAIR_322));
const C_PAIR_324: S<'static> = S::Pair(&(C_IF, C_PAIR_323));
const C_PAIR_325: S<'static> = S::Pair(&(C_PAIR_324, S::Empty));
const C_PAIR_326: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_325));
const C_PAIR_327: S<'static> = S::Pair(&(C_STAR_INDUCTION, C_PAIR_326));
const C_PAIR_328: S<'static> = S::Pair(&(C_DEFUN, C_PAIR_327));
const C_PAIR_329: S<'static> = S::Pair(&(C_PAIR_328, C_PAIR_315));
const C_PAIR_330: S<'static> = S::Pair(&(C_PAIR_329, S::Empty));
const C_PAIR_331: S<'static> = S::Pair(&(C_PAIR_20, C_PAIR_310));
const C_PAIR_332: S<'static> = S::Pair(&(C_PAIR_331, C_PAIR_300));
const C_PAIR_333: S<'static> = S::Pair(&(C_PAIR_123, C_PAIR_332));
const C_PAIR_334: S<'static> = S::Pair(&(C_LIST_INDUCTION, C_PAIR_126));
const C_PAIR_335: S<'static> = S::Pair(&(C_PAIR_334, S::Empty));
const C_PAIR_336: S<'static> = S::Pair(&(C_PAIR_142, C_PAIR_335));
const C_PAIR_337: S<'static> = S::Pair(&(C_CONS, C_PAIR_336));
const C_PAIR_338: S<'static> = S::Pair(&(C_PAIR_337, S::Empty));
const C_PAIR_339: S<'static> = S::Pair(&(S::Empty, S::Empty));
const C_PAIR_340: S<'static> = S::Pair(&(C_QUOTE, C_PAIR_339));
const C_PAIR_341: S<'static> = S::Pair(&(C_PAIR_340, C_PAIR_338));
const C_PAIR_342: S<'static> = S::Pair(&(C_PAIR_134, C_PAIR_341));
const C_PAIR_343: S<'static> = S::Pair(&(C_IF, C_PAIR_342));
const C_PAIR_344: S<'static> = S::Pair(&(C_PAIR_343, S::Empty));
const C_PAIR_345: S<'static> = S::Pair(&(C_PAIR_14, C_PAIR_344));
const C_PAIR_346: S<'static> = S::Pair(&(C_LIST_INDUCTION, C_PAIR_345));
const C_PAIR_347: S<'static> = S::Pair(&(C_DEFUN, C_PAIR_346));
const C_PAIR_348: S<'static> = S::Pair(&(C_PAIR_347, C_PAIR_333));
const C_PAIR_349: S<'static> = S::Pair(&(C_PAIR_348, C_PAIR_330));

pub fn list0<'a>(context: &'a Context<'a>) -> S<'a> {
    S::Empty
}

pub fn is_list0<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    equal(context, x, S::Empty)
}

pub fn list1<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    cons(context, x, list0(context))
}

pub fn is_list1<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        is_list0(context, cdr(context, x))
    }
}

pub fn elem1<'a>(context: &'a Context<'a>, xs: S<'a>) -> S<'a> {
    car(context, xs)
}

pub fn list2<'a>(context: &'a Context<'a>, x: S<'a>, y: S<'a>) -> S<'a> {
    cons(context, x, list1(context, y))
}

pub fn is_list2<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        is_list1(context, cdr(context, x))
    }
}

pub fn elem2<'a>(context: &'a Context<'a>, xs: S<'a>) -> S<'a> {
    elem1(context, cdr(context, xs))
}

pub fn list3<'a>(context: &'a Context<'a>, x: S<'a>, y: S<'a>, z: S<'a>) -> S<'a> {
    cons(context, x, list2(context, y, z))
}

pub fn is_list3<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        is_list2(context, cdr(context, x))
    }
}

pub fn elem3<'a>(context: &'a Context<'a>, xs: S<'a>) -> S<'a> {
    elem2(context, cdr(context, xs))
}

pub fn tag<'a>(context: &'a Context<'a>, sym: S<'a>, x: S<'a>) -> S<'a> {
    cons(context, sym, x)
}

pub fn is_tag<'a>(context: &'a Context<'a>, sym: S<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        equal(context, car(context, x), sym)
    }
}

pub fn untag<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    cdr(context, x)
}

pub fn is_member<'a>(context: &'a Context<'a>, x: S<'a>, ys: S<'a>) -> S<'a> {
    if atom(context, ys) != C_NIL {
        C_NIL
    } else {
        if equal(context, x, car(context, ys)) != C_NIL {
            C_T
        } else {
            is_member(context, x, cdr(context, ys))
        }
    }
}

pub fn quote_c<'a>(context: &'a Context<'a>, value: S<'a>) -> S<'a> {
    tag(context, C_QUOTE, list1(context, value))
}

pub fn is_quote<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_QUOTE, x) != C_NIL {
        is_list1(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn quote_dot_value<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    elem1(context, untag(context, e))
}

pub fn if_c<'a>(context: &'a Context<'a>, q: S<'a>, a: S<'a>, e: S<'a>) -> S<'a> {
    tag(context, C_IF, list3(context, q, a, e))
}

pub fn is_if<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_IF, x) != C_NIL {
        is_list3(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn if_dot_q<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    elem1(context, untag(context, e))
}

pub fn if_dot_a<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    elem2(context, untag(context, e))
}

pub fn if_dot_e<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    elem3(context, untag(context, e))
}

pub fn app_c<'a>(context: &'a Context<'a>, name: S<'a>, args: S<'a>) -> S<'a> {
    cons(context, name, args)
}

pub fn is_app<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if atom(context, x) != C_NIL {
        C_NIL
    } else {
        if is_quote(context, x) != C_NIL {
            C_NIL
        } else {
            if is_if(context, x) != C_NIL {
                C_NIL
            } else {
                C_T
            }
        }
    }
}

pub fn app_dot_name<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    car(context, e)
}

pub fn app_dot_args<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    cdr(context, e)
}

pub fn is_var<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if equal(context, x, C_T) != C_NIL {
        C_NIL
    } else {
        if equal(context, x, C_NIL) != C_NIL {
            C_NIL
        } else {
            if natp(context, x) != C_NIL {
                C_NIL
            } else {
                atom(context, x)
            }
        }
    }
}

pub fn defun_c<'a>(context: &'a Context<'a>, name: S<'a>, formals: S<'a>, body: S<'a>) -> S<'a> {
    tag(context, C_DEFUN, list3(context, name, formals, body))
}

pub fn is_defun<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_DEFUN, x) != C_NIL {
        is_list3(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn defun_dot_name<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    elem1(context, untag(context, def))
}

pub fn defun_dot_formals<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    elem2(context, untag(context, def))
}

pub fn defun_dot_body<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    elem3(context, untag(context, def))
}

pub fn dethm_c<'a>(context: &'a Context<'a>, name: S<'a>, formals: S<'a>, body: S<'a>) -> S<'a> {
    tag(context, C_DETHM, list3(context, name, formals, body))
}

pub fn is_dethm<'a>(context: &'a Context<'a>, x: S<'a>) -> S<'a> {
    if is_tag(context, C_DETHM, x) != C_NIL {
        is_list3(context, untag(context, x))
    } else {
        C_NIL
    }
}

pub fn dethm_dot_name<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    elem1(context, untag(context, def))
}

pub fn dethm_dot_formals<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    elem2(context, untag(context, def))
}

pub fn dethm_dot_body<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    elem3(context, untag(context, def))
}

pub fn if_qae<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    list3(
        context,
        if_dot_q(context, e),
        if_dot_a(context, e),
        if_dot_e(context, e),
    )
}

pub fn qae_if<'a>(context: &'a Context<'a>, es: S<'a>) -> S<'a> {
    if_c(
        context,
        elem1(context, es),
        elem2(context, es),
        elem3(context, es),
    )
}

pub fn is_rator<'a>(context: &'a Context<'a>, name: S<'a>) -> S<'a> {
    is_member(context, name, C_PAIR_8)
}

pub fn rator_dot_formals<'a>(context: &'a Context<'a>, rator: S<'a>) -> S<'a> {
    if is_member(context, rator, C_PAIR_13) != C_NIL {
        C_PAIR_14
    } else {
        if is_member(context, rator, C_PAIR_16) != C_NIL {
            C_PAIR_18
        } else {
            C_NIL
        }
    }
}

pub fn def_dot_name<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        defun_dot_name(context, def)
    } else {
        if is_dethm(context, def) != C_NIL {
            dethm_dot_name(context, def)
        } else {
            def
        }
    }
}

pub fn def_dot_formals<'a>(context: &'a Context<'a>, def: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        dethm_dot_formals(context, def)
    } else {
        if is_defun(context, def) != C_NIL {
            defun_dot_formals(context, def)
        } else {
            S::Empty
        }
    }
}

pub fn if_c_when_necessary<'a>(context: &'a Context<'a>, q: S<'a>, a: S<'a>, e: S<'a>) -> S<'a> {
    if equal(context, a, e) != C_NIL {
        a
    } else {
        if_c(context, q, a, e)
    }
}

pub fn conjunction<'a>(context: &'a Context<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        quote_c(context, C_T)
    } else {
        if atom(context, cdr(context, es)) != C_NIL {
            car(context, es)
        } else {
            if_c(
                context,
                car(context, es),
                conjunction(context, cdr(context, es)),
                quote_c(context, C_NIL),
            )
        }
    }
}

pub fn implication<'a>(context: &'a Context<'a>, es: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        e
    } else {
        if_c(
            context,
            car(context, es),
            implication(context, cdr(context, es), e),
            quote_c(context, C_T),
        )
    }
}

pub fn is_args_arity<'a>(context: &'a Context<'a>, def: S<'a>, args: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        C_NIL
    } else {
        if is_defun(context, def) != C_NIL {
            is_arity(context, defun_dot_formals(context, def), args)
        } else {
            if is_rator(context, def) != C_NIL {
                is_arity(context, rator_dot_formals(context, def), args)
            } else {
                C_NIL
            }
        }
    }
}

pub fn is_app_arity<'a>(context: &'a Context<'a>, defs: S<'a>, app: S<'a>) -> S<'a> {
    is_args_arity(
        context,
        lookup(context, app_dot_name(context, app), defs),
        app_dot_args(context, app),
    )
}

pub fn lookup<'a>(context: &'a Context<'a>, name: S<'a>, defs: S<'a>) -> S<'a> {
    if atom(context, defs) != C_NIL {
        name
    } else {
        if equal(context, def_dot_name(context, car(context, defs)), name) != C_NIL {
            car(context, defs)
        } else {
            lookup(context, name, cdr(context, defs))
        }
    }
}

pub fn is_undefined<'a>(context: &'a Context<'a>, name: S<'a>, defs: S<'a>) -> S<'a> {
    if is_var(context, name) != C_NIL {
        equal(context, lookup(context, name, defs), name)
    } else {
        C_NIL
    }
}

pub fn is_bound<'a>(context: &'a Context<'a>, var: S<'a>, vars: S<'a>) -> S<'a> {
    if equal(context, vars, C_ANY) != C_NIL {
        C_T
    } else {
        is_member(context, var, vars)
    }
}

pub fn is_exprs<'a>(context: &'a Context<'a>, defs: S<'a>, vars: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        C_T
    } else {
        if is_var(context, car(context, es)) != C_NIL {
            if is_bound(context, car(context, es), vars) != C_NIL {
                is_exprs(context, defs, vars, cdr(context, es))
            } else {
                C_NIL
            }
        } else {
            if is_quote(context, car(context, es)) != C_NIL {
                is_exprs(context, defs, vars, cdr(context, es))
            } else {
                if is_if(context, car(context, es)) != C_NIL {
                    if is_exprs(context, defs, vars, if_qae(context, car(context, es))) != C_NIL {
                        is_exprs(context, defs, vars, cdr(context, es))
                    } else {
                        C_NIL
                    }
                } else {
                    if is_app(context, car(context, es)) != C_NIL {
                        if is_app_arity(context, defs, car(context, es)) != C_NIL {
                            if is_exprs(
                                context,
                                defs,
                                vars,
                                app_dot_args(context, car(context, es)),
                            ) != C_NIL
                            {
                                is_exprs(context, defs, vars, cdr(context, es))
                            } else {
                                C_NIL
                            }
                        } else {
                            C_NIL
                        }
                    } else {
                        C_NIL
                    }
                }
            }
        }
    }
}

pub fn is_expr<'a>(context: &'a Context<'a>, defs: S<'a>, vars: S<'a>, e: S<'a>) -> S<'a> {
    is_exprs(context, defs, vars, list1(context, e))
}

pub fn is_subset<'a>(context: &'a Context<'a>, xs: S<'a>, ys: S<'a>) -> S<'a> {
    if atom(context, xs) != C_NIL {
        C_T
    } else {
        if is_member(context, car(context, xs), ys) != C_NIL {
            is_subset(context, cdr(context, xs), ys)
        } else {
            C_NIL
        }
    }
}

pub fn list_extend<'a>(context: &'a Context<'a>, xs: S<'a>, x: S<'a>) -> S<'a> {
    if atom(context, xs) != C_NIL {
        list1(context, x)
    } else {
        if equal(context, car(context, xs), x) != C_NIL {
            xs
        } else {
            cons(
                context,
                car(context, xs),
                list_extend(context, cdr(context, xs), x),
            )
        }
    }
}

pub fn list_union<'a>(context: &'a Context<'a>, xs: S<'a>, ys: S<'a>) -> S<'a> {
    if atom(context, ys) != C_NIL {
        xs
    } else {
        list_union(
            context,
            list_extend(context, xs, car(context, ys)),
            cdr(context, ys),
        )
    }
}

pub fn get_arg_from<'a>(context: &'a Context<'a>, n: S<'a>, args: S<'a>, from: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        C_NIL
    } else {
        if equal(context, n, from) != C_NIL {
            car(context, args)
        } else {
            get_arg_from(
                context,
                n,
                cdr(context, args),
                _plus(context, from, S::Num(1)),
            )
        }
    }
}

pub fn get_arg<'a>(context: &'a Context<'a>, n: S<'a>, args: S<'a>) -> S<'a> {
    get_arg_from(context, n, args, S::Num(1))
}

pub fn set_arg_from<'a>(
    context: &'a Context<'a>,
    n: S<'a>,
    args: S<'a>,
    y: S<'a>,
    from: S<'a>,
) -> S<'a> {
    if atom(context, args) != C_NIL {
        S::Empty
    } else {
        if equal(context, n, from) != C_NIL {
            cons(context, y, cdr(context, args))
        } else {
            cons(
                context,
                car(context, args),
                set_arg_from(
                    context,
                    n,
                    cdr(context, args),
                    y,
                    _plus(context, from, S::Num(1)),
                ),
            )
        }
    }
}

pub fn set_arg<'a>(context: &'a Context<'a>, n: S<'a>, args: S<'a>, y: S<'a>) -> S<'a> {
    set_arg_from(context, n, args, y, S::Num(1))
}

pub fn _lt__eq_len_from<'a>(context: &'a Context<'a>, n: S<'a>, args: S<'a>, from: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        C_NIL
    } else {
        if equal(context, n, from) != C_NIL {
            C_T
        } else {
            _lt__eq_len_from(
                context,
                n,
                cdr(context, args),
                _plus(context, from, S::Num(1)),
            )
        }
    }
}

pub fn _lt__eq_len<'a>(context: &'a Context<'a>, n: S<'a>, args: S<'a>) -> S<'a> {
    if _lt(context, S::Num(0), n) != C_NIL {
        _lt__eq_len_from(context, n, args, S::Num(1))
    } else {
        C_NIL
    }
}

pub fn is_arity<'a>(context: &'a Context<'a>, vars: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, vars) != C_NIL {
        atom(context, es)
    } else {
        if atom(context, es) != C_NIL {
            C_NIL
        } else {
            is_arity(context, cdr(context, vars), cdr(context, es))
        }
    }
}

pub fn is_formals<'a>(context: &'a Context<'a>, vars: S<'a>) -> S<'a> {
    if atom(context, vars) != C_NIL {
        C_T
    } else {
        if is_var(context, car(context, vars)) != C_NIL {
            if is_member(context, car(context, vars), cdr(context, vars)) != C_NIL {
                C_NIL
            } else {
                is_formals(context, cdr(context, vars))
            }
        } else {
            C_NIL
        }
    }
}

pub fn is_direction<'a>(context: &'a Context<'a>, dir: S<'a>) -> S<'a> {
    if natp(context, dir) != C_NIL {
        C_T
    } else {
        is_member(context, dir, C_PAIR_21)
    }
}

pub fn is_path<'a>(context: &'a Context<'a>, path: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_T
    } else {
        if is_direction(context, car(context, path)) != C_NIL {
            is_path(context, cdr(context, path))
        } else {
            C_NIL
        }
    }
}

pub fn is_quoted_exprs<'a>(context: &'a Context<'a>, args: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        C_T
    } else {
        if is_quote(context, car(context, args)) != C_NIL {
            is_quoted_exprs(context, cdr(context, args))
        } else {
            C_NIL
        }
    }
}

pub fn is_step_args<'a>(context: &'a Context<'a>, defs: S<'a>, def: S<'a>, args: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        if is_arity(context, dethm_dot_formals(context, def), args) != C_NIL {
            is_exprs(context, defs, C_ANY, args)
        } else {
            C_NIL
        }
    } else {
        if is_defun(context, def) != C_NIL {
            if is_arity(context, defun_dot_formals(context, def), args) != C_NIL {
                is_exprs(context, defs, C_ANY, args)
            } else {
                C_NIL
            }
        } else {
            if is_rator(context, def) != C_NIL {
                if is_arity(context, rator_dot_formals(context, def), args) != C_NIL {
                    is_quoted_exprs(context, args)
                } else {
                    C_NIL
                }
            } else {
                C_NIL
            }
        }
    }
}

pub fn is_step_app<'a>(context: &'a Context<'a>, defs: S<'a>, app: S<'a>) -> S<'a> {
    is_step_args(
        context,
        defs,
        lookup(context, app_dot_name(context, app), defs),
        app_dot_args(context, app),
    )
}

pub fn is_step<'a>(context: &'a Context<'a>, defs: S<'a>, step: S<'a>) -> S<'a> {
    if is_path(context, elem1(context, step)) != C_NIL {
        if is_app(context, elem2(context, step)) != C_NIL {
            is_step_app(context, defs, elem2(context, step))
        } else {
            C_NIL
        }
    } else {
        C_NIL
    }
}

pub fn is_steps<'a>(context: &'a Context<'a>, defs: S<'a>, steps: S<'a>) -> S<'a> {
    if atom(context, steps) != C_NIL {
        C_T
    } else {
        if is_step(context, defs, car(context, steps)) != C_NIL {
            is_steps(context, defs, cdr(context, steps))
        } else {
            C_NIL
        }
    }
}

pub fn is_induction_scheme_for<'a>(
    context: &'a Context<'a>,
    def: S<'a>,
    vars: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        if is_arity(
            context,
            defun_dot_formals(context, def),
            app_dot_args(context, e),
        ) != C_NIL
        {
            if is_formals(context, app_dot_args(context, e)) != C_NIL {
                is_subset(context, app_dot_args(context, e), vars)
            } else {
                C_NIL
            }
        } else {
            C_NIL
        }
    } else {
        C_NIL
    }
}

pub fn is_induction_scheme<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    vars: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_app(context, e) != C_NIL {
        is_induction_scheme_for(
            context,
            lookup(context, app_dot_name(context, e), defs),
            vars,
            e,
        )
    } else {
        C_NIL
    }
}

pub fn is_seed<'a>(context: &'a Context<'a>, defs: S<'a>, def: S<'a>, seed: S<'a>) -> S<'a> {
    if equal(context, seed, C_NIL) != C_NIL {
        C_T
    } else {
        if is_defun(context, def) != C_NIL {
            is_expr(context, defs, defun_dot_formals(context, def), seed)
        } else {
            if is_dethm(context, def) != C_NIL {
                is_induction_scheme(context, defs, dethm_dot_formals(context, def), seed)
            } else {
                C_NIL
            }
        }
    }
}

pub fn extend_rec<'a>(context: &'a Context<'a>, defs: S<'a>, def: S<'a>) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        list_extend(
            context,
            defs,
            defun_c(
                context,
                defun_dot_name(context, def),
                defun_dot_formals(context, def),
                app_c(
                    context,
                    defun_dot_name(context, def),
                    defun_dot_formals(context, def),
                ),
            ),
        )
    } else {
        defs
    }
}

pub fn is_def_contents<'a>(
    context: &'a Context<'a>,
    known_defs: S<'a>,
    formals: S<'a>,
    body: S<'a>,
) -> S<'a> {
    if is_formals(context, formals) != C_NIL {
        is_expr(context, known_defs, formals, body)
    } else {
        C_NIL
    }
}

pub fn is_def<'a>(context: &'a Context<'a>, known_defs: S<'a>, def: S<'a>) -> S<'a> {
    if is_dethm(context, def) != C_NIL {
        if is_undefined(context, dethm_dot_name(context, def), known_defs) != C_NIL {
            is_def_contents(
                context,
                known_defs,
                dethm_dot_formals(context, def),
                dethm_dot_body(context, def),
            )
        } else {
            C_NIL
        }
    } else {
        if is_defun(context, def) != C_NIL {
            if is_undefined(context, defun_dot_name(context, def), known_defs) != C_NIL {
                is_def_contents(
                    context,
                    extend_rec(context, known_defs, def),
                    defun_dot_formals(context, def),
                    defun_dot_body(context, def),
                )
            } else {
                C_NIL
            }
        } else {
            C_NIL
        }
    }
}

pub fn is_defs<'a>(context: &'a Context<'a>, known_defs: S<'a>, defs: S<'a>) -> S<'a> {
    if atom(context, defs) != C_NIL {
        C_T
    } else {
        if is_def(context, known_defs, car(context, defs)) != C_NIL {
            is_defs(
                context,
                list_extend(context, known_defs, car(context, defs)),
                cdr(context, defs),
            )
        } else {
            C_NIL
        }
    }
}

pub fn is_list2_or_more<'a>(context: &'a Context<'a>, pf: S<'a>) -> S<'a> {
    if atom(context, pf) != C_NIL {
        C_NIL
    } else {
        if atom(context, cdr(context, pf)) != C_NIL {
            C_NIL
        } else {
            C_T
        }
    }
}

pub fn is_proof<'a>(context: &'a Context<'a>, defs: S<'a>, pf: S<'a>) -> S<'a> {
    if is_list2_or_more(context, pf) != C_NIL {
        if is_def(context, defs, elem1(context, pf)) != C_NIL {
            if is_seed(context, defs, elem1(context, pf), elem2(context, pf)) != C_NIL {
                is_steps(
                    context,
                    extend_rec(context, defs, elem1(context, pf)),
                    cdr(context, cdr(context, pf)),
                )
            } else {
                C_NIL
            }
        } else {
            C_NIL
        }
    } else {
        C_NIL
    }
}

pub fn is_proofs<'a>(context: &'a Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if atom(context, pfs) != C_NIL {
        C_T
    } else {
        if is_proof(context, defs, car(context, pfs)) != C_NIL {
            is_proofs(
                context,
                list_extend(context, defs, elem1(context, car(context, pfs))),
                cdr(context, pfs),
            )
        } else {
            C_NIL
        }
    }
}

pub fn sub_var<'a>(context: &'a Context<'a>, vars: S<'a>, args: S<'a>, var: S<'a>) -> S<'a> {
    if atom(context, vars) != C_NIL {
        var
    } else {
        if equal(context, car(context, vars), var) != C_NIL {
            car(context, args)
        } else {
            sub_var(context, cdr(context, vars), cdr(context, args), var)
        }
    }
}

pub fn sub_es<'a>(context: &'a Context<'a>, vars: S<'a>, args: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        S::Empty
    } else {
        if is_var(context, car(context, es)) != C_NIL {
            cons(
                context,
                sub_var(context, vars, args, car(context, es)),
                sub_es(context, vars, args, cdr(context, es)),
            )
        } else {
            if is_quote(context, car(context, es)) != C_NIL {
                cons(
                    context,
                    car(context, es),
                    sub_es(context, vars, args, cdr(context, es)),
                )
            } else {
                if is_if(context, car(context, es)) != C_NIL {
                    cons(
                        context,
                        qae_if(
                            context,
                            sub_es(context, vars, args, if_qae(context, car(context, es))),
                        ),
                        sub_es(context, vars, args, cdr(context, es)),
                    )
                } else {
                    cons(
                        context,
                        app_c(
                            context,
                            app_dot_name(context, car(context, es)),
                            sub_es(context, vars, args, app_dot_args(context, car(context, es))),
                        ),
                        sub_es(context, vars, args, cdr(context, es)),
                    )
                }
            }
        }
    }
}

pub fn sub_e<'a>(context: &'a Context<'a>, vars: S<'a>, args: S<'a>, e: S<'a>) -> S<'a> {
    elem1(context, sub_es(context, vars, args, list1(context, e)))
}

pub fn exprs_recs<'a>(context: &'a Context<'a>, f: S<'a>, es: S<'a>) -> S<'a> {
    if atom(context, es) != C_NIL {
        S::Empty
    } else {
        if is_var(context, car(context, es)) != C_NIL {
            exprs_recs(context, f, cdr(context, es))
        } else {
            if is_quote(context, car(context, es)) != C_NIL {
                exprs_recs(context, f, cdr(context, es))
            } else {
                if is_if(context, car(context, es)) != C_NIL {
                    list_union(
                        context,
                        exprs_recs(context, f, if_qae(context, car(context, es))),
                        exprs_recs(context, f, cdr(context, es)),
                    )
                } else {
                    if equal(context, app_dot_name(context, car(context, es)), f) != C_NIL {
                        list_union(
                            context,
                            list1(context, car(context, es)),
                            list_union(
                                context,
                                exprs_recs(context, f, app_dot_args(context, car(context, es))),
                                exprs_recs(context, f, cdr(context, es)),
                            ),
                        )
                    } else {
                        list_union(
                            context,
                            exprs_recs(context, f, app_dot_args(context, car(context, es))),
                            exprs_recs(context, f, cdr(context, es)),
                        )
                    }
                }
            }
        }
    }
}

pub fn expr_recs<'a>(context: &'a Context<'a>, f: S<'a>, e: S<'a>) -> S<'a> {
    exprs_recs(context, f, list1(context, e))
}

pub fn totality_slash__lt<'a>(
    context: &'a Context<'a>,
    meas: S<'a>,
    formals: S<'a>,
    app: S<'a>,
) -> S<'a> {
    app_c(
        context,
        C__LT,
        list2(
            context,
            sub_e(context, formals, app_dot_args(context, app), meas),
            meas,
        ),
    )
}

pub fn totality_slash_meas<'a>(
    context: &'a Context<'a>,
    meas: S<'a>,
    formals: S<'a>,
    apps: S<'a>,
) -> S<'a> {
    if atom(context, apps) != C_NIL {
        S::Empty
    } else {
        cons(
            context,
            totality_slash__lt(context, meas, formals, car(context, apps)),
            totality_slash_meas(context, meas, formals, cdr(context, apps)),
        )
    }
}

pub fn totality_slash_if<'a>(
    context: &'a Context<'a>,
    meas: S<'a>,
    f: S<'a>,
    formals: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_if(context, e) != C_NIL {
        conjunction(
            context,
            list_extend(
                context,
                totality_slash_meas(
                    context,
                    meas,
                    formals,
                    expr_recs(context, f, if_dot_q(context, e)),
                ),
                if_c_when_necessary(
                    context,
                    if_dot_q(context, e),
                    totality_slash_if(context, meas, f, formals, if_dot_a(context, e)),
                    totality_slash_if(context, meas, f, formals, if_dot_e(context, e)),
                ),
            ),
        )
    } else {
        conjunction(
            context,
            totality_slash_meas(context, meas, formals, expr_recs(context, f, e)),
        )
    }
}

pub fn totality_slash_claim<'a>(context: &'a Context<'a>, meas: S<'a>, def: S<'a>) -> S<'a> {
    if equal(context, meas, C_NIL) != C_NIL {
        if equal(
            context,
            expr_recs(
                context,
                defun_dot_name(context, def),
                defun_dot_body(context, def),
            ),
            S::Empty,
        ) != C_NIL
        {
            quote_c(context, C_T)
        } else {
            quote_c(context, C_NIL)
        }
    } else {
        if_c(
            context,
            app_c(context, C_NATP, list1(context, meas)),
            totality_slash_if(
                context,
                meas,
                defun_dot_name(context, def),
                defun_dot_formals(context, def),
                defun_dot_body(context, def),
            ),
            quote_c(context, C_NIL),
        )
    }
}

pub fn induction_slash_prems<'a>(
    context: &'a Context<'a>,
    vars: S<'a>,
    claim: S<'a>,
    apps: S<'a>,
) -> S<'a> {
    if atom(context, apps) != C_NIL {
        S::Empty
    } else {
        cons(
            context,
            sub_e(
                context,
                vars,
                app_dot_args(context, car(context, apps)),
                claim,
            ),
            induction_slash_prems(context, vars, claim, cdr(context, apps)),
        )
    }
}

pub fn induction_slash_if<'a>(
    context: &'a Context<'a>,
    vars: S<'a>,
    claim: S<'a>,
    f: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if is_if(context, e) != C_NIL {
        implication(
            context,
            induction_slash_prems(
                context,
                vars,
                claim,
                expr_recs(context, f, if_dot_q(context, e)),
            ),
            if_c_when_necessary(
                context,
                if_dot_q(context, e),
                induction_slash_if(context, vars, claim, f, if_dot_a(context, e)),
                induction_slash_if(context, vars, claim, f, if_dot_e(context, e)),
            ),
        )
    } else {
        implication(
            context,
            induction_slash_prems(context, vars, claim, expr_recs(context, f, e)),
            claim,
        )
    }
}

pub fn induction_slash_defun<'a>(
    context: &'a Context<'a>,
    vars: S<'a>,
    claim: S<'a>,
    def: S<'a>,
) -> S<'a> {
    induction_slash_if(
        context,
        vars,
        claim,
        defun_dot_name(context, def),
        sub_e(
            context,
            defun_dot_formals(context, def),
            vars,
            defun_dot_body(context, def),
        ),
    )
}

pub fn induction_slash_claim<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    seed: S<'a>,
    def: S<'a>,
) -> S<'a> {
    if equal(context, seed, C_NIL) != C_NIL {
        dethm_dot_body(context, def)
    } else {
        induction_slash_defun(
            context,
            app_dot_args(context, seed),
            dethm_dot_body(context, def),
            lookup(context, app_dot_name(context, seed), defs),
        )
    }
}

pub fn find_focus_at_direction<'a>(context: &'a Context<'a>, dir: S<'a>, e: S<'a>) -> S<'a> {
    if equal(context, dir, C_Q) != C_NIL {
        if_dot_q(context, e)
    } else {
        if equal(context, dir, C_A) != C_NIL {
            if_dot_a(context, e)
        } else {
            if equal(context, dir, C_E) != C_NIL {
                if_dot_e(context, e)
            } else {
                get_arg(context, dir, app_dot_args(context, e))
            }
        }
    }
}

pub fn rewrite_focus_at_direction<'a>(
    context: &'a Context<'a>,
    dir: S<'a>,
    e1: S<'a>,
    e2: S<'a>,
) -> S<'a> {
    if equal(context, dir, C_Q) != C_NIL {
        if_c(context, e2, if_dot_a(context, e1), if_dot_e(context, e1))
    } else {
        if equal(context, dir, C_A) != C_NIL {
            if_c(context, if_dot_q(context, e1), e2, if_dot_e(context, e1))
        } else {
            if equal(context, dir, C_E) != C_NIL {
                if_c(context, if_dot_q(context, e1), if_dot_a(context, e1), e2)
            } else {
                app_c(
                    context,
                    app_dot_name(context, e1),
                    set_arg(context, dir, app_dot_args(context, e1), e2),
                )
            }
        }
    }
}

pub fn is_focus_is_at_direction<'a>(context: &'a Context<'a>, dir: S<'a>, e: S<'a>) -> S<'a> {
    if equal(context, dir, C_Q) != C_NIL {
        is_if(context, e)
    } else {
        if equal(context, dir, C_A) != C_NIL {
            is_if(context, e)
        } else {
            if equal(context, dir, C_E) != C_NIL {
                is_if(context, e)
            } else {
                if is_app(context, e) != C_NIL {
                    _lt__eq_len(context, dir, app_dot_args(context, e))
                } else {
                    C_NIL
                }
            }
        }
    }
}

pub fn is_focus_is_at_path<'a>(context: &'a Context<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_T
    } else {
        if is_focus_is_at_direction(context, car(context, path), e) != C_NIL {
            is_focus_is_at_path(
                context,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e),
            )
        } else {
            C_NIL
        }
    }
}

pub fn find_focus_at_path<'a>(context: &'a Context<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        e
    } else {
        find_focus_at_path(
            context,
            cdr(context, path),
            find_focus_at_direction(context, car(context, path), e),
        )
    }
}

pub fn rewrite_focus_at_path<'a>(
    context: &'a Context<'a>,
    path: S<'a>,
    e1: S<'a>,
    e2: S<'a>,
) -> S<'a> {
    if atom(context, path) != C_NIL {
        e2
    } else {
        rewrite_focus_at_direction(
            context,
            car(context, path),
            e1,
            rewrite_focus_at_path(
                context,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e1),
                e2,
            ),
        )
    }
}

pub fn is_prem_a<'a>(context: &'a Context<'a>, prem: S<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_NIL
    } else {
        if equal(context, car(context, path), C_A) != C_NIL {
            if equal(context, if_dot_q(context, e), prem) != C_NIL {
                C_T
            } else {
                is_prem_a(
                    context,
                    prem,
                    cdr(context, path),
                    find_focus_at_direction(context, car(context, path), e),
                )
            }
        } else {
            is_prem_a(
                context,
                prem,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e),
            )
        }
    }
}

pub fn is_prem_e<'a>(context: &'a Context<'a>, prem: S<'a>, path: S<'a>, e: S<'a>) -> S<'a> {
    if atom(context, path) != C_NIL {
        C_NIL
    } else {
        if equal(context, car(context, path), C_E) != C_NIL {
            if equal(context, if_dot_q(context, e), prem) != C_NIL {
                C_T
            } else {
                is_prem_e(
                    context,
                    prem,
                    cdr(context, path),
                    find_focus_at_direction(context, car(context, path), e),
                )
            }
        } else {
            is_prem_e(
                context,
                prem,
                cdr(context, path),
                find_focus_at_direction(context, car(context, path), e),
            )
        }
    }
}

pub fn follow_prems<'a>(context: &'a Context<'a>, path: S<'a>, e: S<'a>, thm: S<'a>) -> S<'a> {
    if is_if(context, thm) != C_NIL {
        if is_prem_a(context, if_dot_q(context, thm), path, e) != C_NIL {
            follow_prems(context, path, e, if_dot_a(context, thm))
        } else {
            if is_prem_e(context, if_dot_q(context, thm), path, e) != C_NIL {
                follow_prems(context, path, e, if_dot_e(context, thm))
            } else {
                thm
            }
        }
    } else {
        thm
    }
}

pub fn unary_op<'a>(context: &'a Context<'a>, rator: S<'a>, rand: S<'a>) -> S<'a> {
    if equal(context, rator, C_ATOM) != C_NIL {
        atom(context, rand)
    } else {
        if equal(context, rator, C_CAR) != C_NIL {
            car(context, rand)
        } else {
            if equal(context, rator, C_CDR) != C_NIL {
                cdr(context, rand)
            } else {
                if equal(context, rator, C_NATP) != C_NIL {
                    natp(context, rand)
                } else {
                    if equal(context, rator, C_SIZE) != C_NIL {
                        size(context, rand)
                    } else {
                        C_NIL
                    }
                }
            }
        }
    }
}

pub fn binary_op<'a>(context: &'a Context<'a>, rator: S<'a>, rand1: S<'a>, rand2: S<'a>) -> S<'a> {
    if equal(context, rator, C_EQUAL) != C_NIL {
        equal(context, rand1, rand2)
    } else {
        if equal(context, rator, C_CONS) != C_NIL {
            cons(context, rand1, rand2)
        } else {
            if equal(context, rator, C__PLUS) != C_NIL {
                _plus(context, rand1, rand2)
            } else {
                if equal(context, rator, C__LT) != C_NIL {
                    _lt(context, rand1, rand2)
                } else {
                    C_NIL
                }
            }
        }
    }
}

pub fn apply_op<'a>(context: &'a Context<'a>, rator: S<'a>, rands: S<'a>) -> S<'a> {
    if is_member(context, rator, C_PAIR_13) != C_NIL {
        unary_op(context, rator, elem1(context, rands))
    } else {
        if is_member(context, rator, C_PAIR_16) != C_NIL {
            binary_op(context, rator, elem1(context, rands), elem2(context, rands))
        } else {
            C_NIL
        }
    }
}

pub fn rands<'a>(context: &'a Context<'a>, args: S<'a>) -> S<'a> {
    if atom(context, args) != C_NIL {
        S::Empty
    } else {
        cons(
            context,
            quote_dot_value(context, car(context, args)),
            rands(context, cdr(context, args)),
        )
    }
}

pub fn eval_op<'a>(context: &'a Context<'a>, app: S<'a>) -> S<'a> {
    quote_c(
        context,
        apply_op(
            context,
            app_dot_name(context, app),
            rands(context, app_dot_args(context, app)),
        ),
    )
}

pub fn is_app_of_equal<'a>(context: &'a Context<'a>, e: S<'a>) -> S<'a> {
    if is_app(context, e) != C_NIL {
        equal(context, app_dot_name(context, e), C_EQUAL)
    } else {
        C_NIL
    }
}

pub fn equality<'a>(context: &'a Context<'a>, focus: S<'a>, a: S<'a>, b: S<'a>) -> S<'a> {
    if equal(context, focus, a) != C_NIL {
        b
    } else {
        if equal(context, focus, b) != C_NIL {
            a
        } else {
            focus
        }
    }
}

pub fn equality_slash_equation<'a>(
    context: &'a Context<'a>,
    focus: S<'a>,
    concl_inst: S<'a>,
) -> S<'a> {
    if is_app_of_equal(context, concl_inst) != C_NIL {
        equality(
            context,
            focus,
            elem1(context, app_dot_args(context, concl_inst)),
            elem2(context, app_dot_args(context, concl_inst)),
        )
    } else {
        focus
    }
}

pub fn equality_slash_path<'a>(
    context: &'a Context<'a>,
    e: S<'a>,
    path: S<'a>,
    thm: S<'a>,
) -> S<'a> {
    if is_focus_is_at_path(context, path, e) != C_NIL {
        rewrite_focus_at_path(
            context,
            path,
            e,
            equality_slash_equation(
                context,
                find_focus_at_path(context, path, e),
                follow_prems(context, path, e, thm),
            ),
        )
    } else {
        e
    }
}

pub fn equality_slash_def<'a>(
    context: &'a Context<'a>,
    claim: S<'a>,
    path: S<'a>,
    app: S<'a>,
    def: S<'a>,
) -> S<'a> {
    if is_rator(context, def) != C_NIL {
        equality_slash_path(
            context,
            claim,
            path,
            app_c(context, C_EQUAL, list2(context, app, eval_op(context, app))),
        )
    } else {
        if is_defun(context, def) != C_NIL {
            equality_slash_path(
                context,
                claim,
                path,
                sub_e(
                    context,
                    defun_dot_formals(context, def),
                    app_dot_args(context, app),
                    app_c(
                        context,
                        C_EQUAL,
                        list2(
                            context,
                            app_c(
                                context,
                                defun_dot_name(context, def),
                                defun_dot_formals(context, def),
                            ),
                            defun_dot_body(context, def),
                        ),
                    ),
                ),
            )
        } else {
            if is_dethm(context, def) != C_NIL {
                equality_slash_path(
                    context,
                    claim,
                    path,
                    sub_e(
                        context,
                        dethm_dot_formals(context, def),
                        app_dot_args(context, app),
                        dethm_dot_body(context, def),
                    ),
                )
            } else {
                claim
            }
        }
    }
}

pub fn rewrite_slash_step<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    claim: S<'a>,
    step: S<'a>,
) -> S<'a> {
    equality_slash_def(
        context,
        claim,
        elem1(context, step),
        elem2(context, step),
        lookup(context, app_dot_name(context, elem2(context, step)), defs),
    )
}

pub fn rewrite_slash_continue<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    steps: S<'a>,
    old: S<'a>,
    new: S<'a>,
) -> S<'a> {
    if equal(context, new, old) != C_NIL {
        new
    } else {
        if atom(context, steps) != C_NIL {
            new
        } else {
            rewrite_slash_continue(
                context,
                defs,
                cdr(context, steps),
                new,
                rewrite_slash_step(context, defs, new, car(context, steps)),
            )
        }
    }
}

pub fn rewrite_slash_steps<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    claim: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if atom(context, steps) != C_NIL {
        claim
    } else {
        rewrite_slash_continue(
            context,
            defs,
            cdr(context, steps),
            claim,
            rewrite_slash_step(context, defs, claim, car(context, steps)),
        )
    }
}

pub fn rewrite_slash_prove<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    def: S<'a>,
    seed: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if is_defun(context, def) != C_NIL {
        rewrite_slash_steps(
            context,
            defs,
            totality_slash_claim(context, seed, def),
            steps,
        )
    } else {
        if is_dethm(context, def) != C_NIL {
            rewrite_slash_steps(
                context,
                defs,
                induction_slash_claim(context, defs, seed, def),
                steps,
            )
        } else {
            quote_c(context, C_NIL)
        }
    }
}

pub fn rewrite_slash_prove_plus_1<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    pf: S<'a>,
    e: S<'a>,
) -> S<'a> {
    if equal(context, e, quote_c(context, C_T)) != C_NIL {
        rewrite_slash_prove(
            context,
            defs,
            elem1(context, pf),
            elem2(context, pf),
            cdr(context, cdr(context, pf)),
        )
    } else {
        e
    }
}

pub fn rewrite_slash_prove_plus<'a>(context: &'a Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if atom(context, pfs) != C_NIL {
        quote_c(context, C_T)
    } else {
        rewrite_slash_prove_plus_1(
            context,
            defs,
            car(context, pfs),
            rewrite_slash_prove_plus(
                context,
                list_extend(context, defs, elem1(context, car(context, pfs))),
                cdr(context, pfs),
            ),
        )
    }
}

pub fn rewrite_slash_define<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    def: S<'a>,
    seed: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if equal(
        context,
        rewrite_slash_prove(context, defs, def, seed, steps),
        quote_c(context, C_T),
    ) != C_NIL
    {
        list_extend(context, defs, def)
    } else {
        defs
    }
}

pub fn rewrite_slash_define_plus_1<'a>(
    context: &'a Context<'a>,
    defs1: S<'a>,
    defs2: S<'a>,
    pfs: S<'a>,
) -> S<'a> {
    if equal(context, defs1, defs2) != C_NIL {
        defs1
    } else {
        if atom(context, pfs) != C_NIL {
            defs2
        } else {
            rewrite_slash_define_plus_1(
                context,
                defs2,
                rewrite_slash_define(
                    context,
                    defs2,
                    elem1(context, car(context, pfs)),
                    elem2(context, car(context, pfs)),
                    cdr(context, cdr(context, car(context, pfs))),
                ),
                cdr(context, pfs),
            )
        }
    }
}

pub fn rewrite_slash_define_plus<'a>(context: &'a Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if atom(context, pfs) != C_NIL {
        defs
    } else {
        rewrite_slash_define_plus_1(
            context,
            defs,
            rewrite_slash_define(
                context,
                defs,
                elem1(context, car(context, pfs)),
                elem2(context, car(context, pfs)),
                cdr(context, cdr(context, car(context, pfs))),
            ),
            cdr(context, pfs),
        )
    }
}

pub fn j_bob_slash_step<'a>(
    context: &'a Context<'a>,
    defs: S<'a>,
    e: S<'a>,
    steps: S<'a>,
) -> S<'a> {
    if is_defs(context, S::Empty, defs) != C_NIL {
        if is_expr(context, defs, C_ANY, e) != C_NIL {
            if is_steps(context, defs, steps) != C_NIL {
                rewrite_slash_steps(context, defs, e, steps)
            } else {
                e
            }
        } else {
            e
        }
    } else {
        e
    }
}

pub fn j_bob_slash_prove<'a>(context: &'a Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if is_defs(context, S::Empty, defs) != C_NIL {
        if is_proofs(context, defs, pfs) != C_NIL {
            rewrite_slash_prove_plus(context, defs, pfs)
        } else {
            quote_c(context, C_NIL)
        }
    } else {
        quote_c(context, C_NIL)
    }
}

pub fn j_bob_slash_define<'a>(context: &'a Context<'a>, defs: S<'a>, pfs: S<'a>) -> S<'a> {
    if is_defs(context, S::Empty, defs) != C_NIL {
        if is_proofs(context, defs, pfs) != C_NIL {
            rewrite_slash_define_plus(context, defs, pfs)
        } else {
            defs
        }
    } else {
        defs
    }
}

pub fn axioms<'a>(context: &'a Context<'a>) -> S<'a> {
    C_PAIR_284
}

pub fn prelude<'a>(context: &'a Context<'a>) -> S<'a> {
    j_bob_slash_define(context, axioms(context), C_PAIR_349)
}
