use unsafe_hacspec_examples::ntt::*;

#[test]
fn monty_kat() {
    let a = -79255;
    let r = montgomery_reduce(a);
    assert_eq!(-1528, r);
    let a = 1275285;
    let r = montgomery_reduce(a);
    assert_eq!(376, r);
}

#[test]
fn ntt_kat() {
    let p = Poly::from_native_slice(&[
        0, 0, -1, -1, 0, 0, 0, -1, 1, 1, 0, 1, 0, 0, -2, 1, -1, -1, -1, 1, -1, 0, 0, 1, 0, 0, 0,
        -1, 0, 0, 1, 1, -1, 2, 2, 1, 1, -1, 1, 1, 0, 2, 0, -1, 0, 0, 0, 0, 0, 0, -2, 0, 2, 0, 0,
        -1, 0, -1, 0, 1, 1, 0, 0, -1, 2, -1, -2, 0, 0, 1, 0, 0, 1, 0, 0, -1, -1, -1, -1, -1, 2, 0,
        1, 0, -1, -1, 0, -2, -1, 0, -1, 1, 0, -1, 0, 2, -1, 0, 2, -1, 0, 1, 1, 0, 0, -1, 1, 1, 1,
        1, 1, 0, 0, -1, 0, 2, 0, -1, 1, 1, 0, 1, 0, 1, -2, 0, 0, -1, 0, -1, 2, 0, 0, 0, -1, 0, 2,
        1, -1, 0, -1, 0, 1, 1, -1, 0, 2, -1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, -1, 0, 0, 0, 1, 0, 1,
        -1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, 1, -1, 2, 0, 1, 1, 0, 0, 0, 0, 0, 1,
        0, 1, 0, 1, -1, 0, -1, -1, 0, -1, -1, 2, 0, 0, -1, 0, 0, -1, 1, -2, 1, 0, 2, 0, -1, -1, 1,
        0, -1, 0, 0, 0, 0, 0, 0, 0, 1, 1, -1, -1, -1, -1, 0, 1, 2, -2, 0, 0, 0, 0, -1, 0, 1, 0, -2,
        0, -1, 0, 1, 1, -2, -1,
    ]);
    let ntt_out_expected = Poly::from_native_slice(&[
        -2840, 2202, -3472, -392, -3991, -372, -1561, -3570, -3844, 1862, -3230, -1320, -1251,
        -1592, -1979, -1834, -1197, 973, -4459, 2685, -6545, 2240, -3711, 4858, -2843, 2998, -107,
        3058, -898, 1843, -1400, 1321, 2483, 3457, 2889, 4217, 1008, 4637, 1836, 7305, 1973, 3563,
        -795, 4309, -358, 2339, -3900, 2597, -1033, 5968, -2757, 5042, -2100, 5086, -5314, 3076,
        -2481, 4956, 213, 1944, 3384, 1631, 344, -1199, -160, 900, 3142, 1726, 3369, 1900, 1613,
        2186, 2417, 3536, 645, 3348, 5319, 6855, 3527, 4109, -943, 3737, 1535, 1903, 376, 369,
        -2900, -545, 1845, 960, 4063, 1460, 179, 2628, 3429, -656, 19, -363, 553, -3417, 1634,
        -1293, 1866, -467, 132, 4637, -226, 1759, 1714, -406, 1356, 2334, 30, 1682, 1548, -1556,
        2570, 1062, -508, -960, 1207, -1742, 1521, -980, 3108, -1487, 3956, -4707, 583, -3838,
        1433, -5394, 3321, -2808, 2283, -72, -101, 453, -1741, 251, 1151, -2390, -1065, -3562, 50,
        -1209, -1178, -2807, 1331, -248, -19, 2812, 1261, -4327, 3117, -3923, 504, -1557, -290,
        -885, 807, -2169, -1027, -2345, -1811, -433, -981, 2387, -640, -553, 2272, -1949, 3051,
        -4178, 185, -3424, -4490, -5583, -4560, -4581, -406, -6155, -2840, -4797, -697, -4346,
        -3719, -2738, 1954, -2116, -298, -1404, 995, -3382, -201, -4116, -1456, -5539, -1306,
        -4591, -4000, -2847, -3640, -67, -1567, 425, 167, -2507, -1155, -2055, -4549, -4535, -3031,
        -1831, -2693, -843, 1009, -924, 1603, -2416, -1606, -1641, -938, -3515, 3292, 543, 2204,
        2721, 1044, -123, 2864, 119, 5506, -878, 3022, 822, 2669, -1479, 3215, -21, 1767, 1527,
        -647, 4683, -2325, 1812, -1559, 354, 1131, 1273, 2871, 1265, 42, 311, -168, -1057,
    ]);

    let ntt_out = ntt(p);
    assert_eq!(ntt_out_expected, ntt_out);
}

#[test]
fn ntt_inv_kat() {
    let p = Poly::from_native_slice(&[
        425, 680, 511, 2471, 246, 2305, 781, 849, 2801, 513, 1391, 3117, 1707, 3279, 2169, 336,
        1845, 96, 2886, 2135, 2753, 1622, 2282, 2646, 124, 2405, 2994, 1843, 472, 2170, 1058, 2941,
        485, 3118, 1083, 98, 1389, 1985, 337, 1926, 2775, 2239, 1906, 533, 1355, 2193, 1786, 1211,
        2338, 2983, 2727, 3235, 2755, 2171, 182, 1955, 2076, 506, 208, 2657, 1476, 220, 1408, 2920,
        227, 2342, 832, 1171, 439, 2130, 1346, 1775, 326, 2613, 2624, 745, 191, 214, 494, 293,
        2487, 181, 2979, 2755, 2244, 1372, 795, 356, 1263, 3200, 971, 1114, 2226, 1000, 2662, 2850,
        1414, 1205, 2914, 104, 1374, 1513, 1969, 2869, 2822, 83, 2819, 1918, 2852, 1088, 33, 514,
        1319, 1906, 2144, 2375, 1415, 2668, 1677, 1682, 2514, 1904, 756, 471, 652, 3325, 3259,
        1114, 1165, 3088, 421, 2890, 2458, 1275, 2518, 2263, 3208, 1004, 3275, 361, 1928, 3176,
        929, 2908, 1532, 3157, 2753, 2377, 751, 1959, 1638, 963, 3276, 2916, 2116, 1906, 1930, 664,
        1802, 1037, 2167, 1452, 1433, 1608, 1903, 2278, 152, 82, 919, 95, 3033, 2325, 715, 3218,
        797, 828, 1777, 1177, 2842, 2716, 1737, 1425, 1888, 3023, 471, 687, 2957, 2722, 2234, 2607,
        3166, 968, 2790, 858, 2800, 562, 1350, 2626, 39, 1304, 2111, 411, 1137, 294, 2917, 1523,
        2910, 2614, 246, 1122, 744, 1648, 1515, 267, 2363, 1526, 2559, 2161, 245, 2841, 3206, 1099,
        506, 1685, 1190, 443, 2072, 1543, 1505, 2855, 421, 2351, 1946, 182, 56, 2358, 3322, 2968,
        1756, 3232, 1293, 266, 660, 1244, 571, 102, 1782, 3177, 2326, 2466, 706, 331, 777, 943, 40,
        3161,
    ]);
    let ntt_inv_out_expected = Poly::from_native_slice(&[
        -1095, 1617, -469, -929, -1260, 283, -194, -57, 1611, 148, -542, 1684, 1529, 1279, 1198,
        -1361, 108, 1644, -1105, -243, 1631, 1176, -898, 1150, -144, -649, -1203, -384, -1648,
        -620, -968, -1237, 1408, -597, -1047, 431, -1435, -724, 466, 1660, 354, -223, 270, 124,
        229, -274, -1044, 1384, 762, 932, -928, -1250, -917, -1529, -1543, 300, 985, 1573, 13,
        1042, 353, -1536, -361, 855, -211, 55, 280, -1284, 433, 605, 1422, 600, 635, -1407, -596,
        749, -830, 58, 1613, -455, -764, -120, -1105, -651, -393, -1270, 81, -1249, 551, 227, -596,
        1013, -550, -1277, 484, -1551, 987, 679, -895, -138, -599, -1477, -1438, -1054, 859, 542,
        1124, -360, 526, -75, 788, 595, -921, 11, 151, 1092, 137, 554, 1071, -108, -314, -1508,
        -963, 176, -1254, 418, -320, -1582, 370, 1021, 1632, -1466, 45, -1374, 1087, -6, 1110,
        1168, -1172, -757, 1425, -111, 156, 730, 1375, -341, 946, -1171, 1036, 177, -1166, -1323,
        -166, -467, -212, 496, 783, -1476, 434, 1529, -1233, -1127, -689, 37, -1529, 71, -1172,
        -1422, -1087, 715, -636, 300, -1555, 1585, -985, 1010, -174, -1133, -763, 466, -495, 780,
        -1608, 20, -489, 1198, -15, -1639, -1424, 699, -1339, 1572, -1164, -136, 537, -224, -478,
        -459, 488, 234, -372, -1160, 79, 335, 966, 1405, -774, 1252, -596, 695, 1369, 432, 1094,
        846, 1602, -277, 1011, 833, -648, -893, -731, -234, 1215, 1428, -784, 1420, -763, 865,
        -915, 376, 65, 1344, 344, 667, -1199, 681, 808, 272, -397, -672, 967, -569, 1559, -481,
        -1219, 883, -1004, -1547, 986, -1528, -172, -353, -691, -962, -1284, -359,
    ]);

    let ntt_inv_out = ntt_inv(p);
    assert_eq!(ntt_inv_out_expected, ntt_inv_out);
}
